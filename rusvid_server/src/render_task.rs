use r2d2::{Pool, PooledConnection};
use redis::{Client, Commands, RedisError};
use rusvid_lib::prelude::Composition;
use rusvid_lib::renderer::embedded::EmbeddedRenderer;
use rusvid_lib::renderer::Renderer;
use s3::Bucket;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::io::{ReaderStream, StreamReader};

use crate::redis::key_for_video_status;
use crate::result_assert;
use crate::status_types::ItemStatus;
use crate::util::{format_file_path, format_s3_file_path};

#[derive(Debug)]
pub struct Message {
    pub composition: Composition,
    pub id: String,
}

async fn render_task(
    id: String,
    composition: Composition,
    bucket: &Bucket,
    connection: &mut PooledConnection<Client>,
) -> Result<String, String> {
    let local_file_path = format_file_path(&id);
    let s3_file_path = format_s3_file_path(&id);

    let mut renderer = EmbeddedRenderer::new(&local_file_path);
    renderer.render(composition).map_err(|err| {
        println!("err: {err:?}");
        local_file_path.clone()
    })?;

    let status: ItemStatus = connection.get(key_for_video_status(&id)).map_err(|err| {
        println!("err: {err:?}");
        local_file_path.clone()
    })?;

    match status {
        ItemStatus::Processing => {
            let file = tokio::fs::File::open(&local_file_path)
                .await
                .map_err(|err| {
                    println!("err: {err:?}");
                    local_file_path.clone()
                })?;

            let stream = ReaderStream::new(file);
            let mut stream_reader = StreamReader::new(stream);

            let response_data = bucket
                .put_object_stream(&mut stream_reader, s3_file_path)
                .await
                .map_err(|err| {
                    println!("err: {err:?}");
                    local_file_path.clone()
                })?;

            result_assert!(response_data == 200, local_file_path.clone())?;

            let _: () = connection
                .set(key_for_video_status(&id), ItemStatus::Finish)
                .map_err(|err| {
                    println!("err: {err:?}");
                    local_file_path.clone()
                })?;
        }
        ItemStatus::Pending => unreachable!(),
        ItemStatus::Finish | ItemStatus::InDeletion | ItemStatus::EncounteredError => (),
    }

    Ok(local_file_path)
}

pub async fn renderer(mut rx: UnboundedReceiver<Message>, bucket: Bucket, pool: Pool<Client>) {
    let mut connection = pool.get().unwrap();

    while let Some(message) = rx.recv().await {
        println!("{}: {:?}", message.id, message.composition);

        let _: () = connection
            .set(key_for_video_status(&message.id), ItemStatus::Processing)
            .expect("Not able to update value in redis to ItemStatus::Processing");

        let render_result = render_task(
            message.id.clone(),
            message.composition,
            &bucket,
            &mut connection,
        )
        .await;

        let local_file_path = match render_result {
            Ok(path) => path,
            Err(path) => {
                let redis_result: Result<(), RedisError> = connection.set(
                    key_for_video_status(&message.id),
                    ItemStatus::EncounteredError,
                );
                if redis_result.is_err() {
                    println!(
                        "encountered error in render_task with the id '{}': {redis_result:?}",
                        message.id
                    );
                }

                path
            }
        };

        tokio::fs::remove_file(local_file_path)
            .await
            .expect("Error in removing file");
    }
}
