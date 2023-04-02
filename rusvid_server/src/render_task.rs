use rusvid_lib::prelude::Composition;
use rusvid_lib::renderer::embedded::EmbeddedRenderer;
use rusvid_lib::renderer::Renderer;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::status_types::{ItemStatus, SharedItemList};
use crate::util::format_file_path;

#[derive(Debug)]
pub struct Message {
    pub composition: Composition,
    pub id: String,
}

pub async fn renderer(mut rx: UnboundedReceiver<Message>, shared_list: SharedItemList) {
    while let Some(message) = rx.recv().await {
        println!("{}: {:?}", message.id, message.composition);

        shared_list
            .write()
            .unwrap()
            .list
            .insert(message.id.clone(), ItemStatus::Processing);

        let mut renderer = EmbeddedRenderer::new(format_file_path(&message.id));
        renderer.render(message.composition).unwrap();

        shared_list
            .write()
            .unwrap()
            .list
            .insert(message.id.clone(), ItemStatus::Finish);
    }
}
