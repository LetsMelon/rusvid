use r2d2::{Pool, PooledConnection};
use redis::{Client, Commands, RedisError};
use rusvid_lib::prelude::Composition;
use rusvid_lib::renderer::embedded::EmbeddedRenderer;
use rusvid_lib::renderer::Renderer;
use s3::Bucket;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::io::{ReaderStream, StreamReader};
use tracing::{debug, error, info, instrument};

use crate::redis::key_for_video_status;
use crate::result_assert;
use crate::status_types::ItemStatus;
use crate::util::{format_file_path, format_s3_file_path};

#[derive(Debug)]
pub struct Message {
    pub composition: Composition,
    pub id: String,
}

#[derive(Debug)]
enum TaskStatus {
    Ok,
    Err(String),
}

#[derive(Debug)]
struct TaskReturn {
    path: String,
    status: TaskStatus,
}

impl TaskReturn {
    fn new(path: String, status: TaskStatus) -> Self {
        TaskReturn { path, status }
    }

    fn new_ok(path: String) -> Self {
        TaskReturn::new(path, TaskStatus::Ok)
    }

    fn new_err(path: String, error_msg: String) -> Self {
        TaskReturn::new(path, TaskStatus::Err(error_msg))
    }

    fn is_ok(&self) -> bool {
        matches!(self.status, TaskStatus::Ok)
    }

    fn is_err(&self) -> bool {
        !self.is_ok()
    }

    fn path(&self) -> &str {
        &self.path
    }
}

macro_rules! return_task_if_err {
    ($r:expr, $p:ident) => {
        match $r {
            Ok(value) => value,
            Err(err) => {
                error!("err: {:?}", err);
                return TaskReturn::new_err($p.clone(), format!("{}", err));
            }
        }
    };
}

#[instrument(skip(composition, bucket, connection))]
async fn render_task(
    id: String,
    composition: Composition,
    bucket: &Bucket,
    connection: &mut PooledConnection<Client>,
) -> TaskReturn {
    let local_file_path = format_file_path(&id);
    let s3_file_path = format_s3_file_path(&id);

    let mut renderer = EmbeddedRenderer::new(&local_file_path);
    return_task_if_err!(renderer.render(composition), local_file_path);

    let status: ItemStatus =
        return_task_if_err!(connection.get(key_for_video_status(&id)), local_file_path);

    match status {
        ItemStatus::Processing => {
            let file = return_task_if_err!(
                tokio::fs::File::open(&local_file_path).await,
                local_file_path
            );

            let stream = ReaderStream::new(file);
            let mut stream_reader = StreamReader::new(stream);

            let response_data = return_task_if_err!(
                bucket
                    .put_object_stream(&mut stream_reader, s3_file_path)
                    .await,
                local_file_path
            );

            return_task_if_err!(
                result_assert!(response_data == 200, local_file_path.clone()),
                local_file_path
            );

            let _: () = return_task_if_err!(
                connection.set(key_for_video_status(&id), ItemStatus::Finish),
                local_file_path
            );
        }
        ItemStatus::Pending => unreachable!(),
        ItemStatus::Finish | ItemStatus::InDeletion | ItemStatus::EncounteredError => (),
    }

    TaskReturn::new_ok(local_file_path)
}

pub async fn renderer(mut rx: UnboundedReceiver<Message>, bucket: Bucket, pool: Pool<Client>) {
    info!("Started renderer");

    let mut connection = pool.get().unwrap();

    while let Some(message) = rx.recv().await {
        info!("Worker got id: {}", message.id);

        let _: () = connection
            .set(key_for_video_status(&message.id), ItemStatus::Processing)
            .expect("Not able to update value in redis to ItemStatus::Processing");
        debug!("Updated resource status");

        info!("Start rendering");
        let render_result = render_task(
            message.id.clone(),
            message.composition,
            &bucket,
            &mut connection,
        )
        .await;

        info!("Finished rendering with status: {:?}", render_result);

        let local_file_path = render_result.path();
        if render_result.is_err() {
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
        }

        tokio::fs::remove_file(local_file_path)
            .await
            .expect("Error in removing file");
    }
}
