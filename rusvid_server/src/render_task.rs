use r2d2_redis::redis::{Commands, RedisResult};
use r2d2_redis::RedisConnectionManager;
use rusvid_lib::prelude::Composition;
use rusvid_lib::renderer::embedded::EmbeddedRenderer;
use rusvid_lib::renderer::Renderer;
use s3::Bucket;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::io::{ReaderStream, StreamReader};

use crate::status_types::ItemStatus;
use crate::util::{format_file_path, format_s3_file_path};

#[derive(Debug)]
pub struct Message {
    pub composition: Composition,
    pub id: String,
}

pub async fn renderer(
    mut rx: UnboundedReceiver<Message>,
    bucket: Bucket,
    pool: r2d2_redis::r2d2::Pool<RedisConnectionManager>,
) {
    let mut connection = pool.get().unwrap();

    while let Some(message) = rx.recv().await {
        println!("{}: {:?}", message.id, message.composition);

        let _: () = connection
            .set(message.id.clone(), ItemStatus::Processing)
            .unwrap();

        let local_file_path = format_file_path(&message.id);
        let s3_file_path = format_s3_file_path(&message.id);

        let mut renderer = EmbeddedRenderer::new(&local_file_path);
        renderer.render(message.composition).unwrap();

        let status: RedisResult<ItemStatus> = connection.get(message.id.clone());
        if let Ok(status) = status {
            if status != ItemStatus::InDeletion {
                let file = tokio::fs::File::open(&local_file_path).await.unwrap();
                let stream = ReaderStream::new(file);
                let mut stream_reader = StreamReader::new(stream);

                let response_data = bucket
                    .put_object_stream(&mut stream_reader, s3_file_path)
                    .await
                    .unwrap();
                assert_eq!(response_data, 200);

                let _: () = connection
                    .set(message.id.clone(), ItemStatus::Finish)
                    .unwrap();
            }
        }

        tokio::fs::remove_file(local_file_path).await.unwrap();
    }
}
