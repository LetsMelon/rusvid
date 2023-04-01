use rusvid_lib::renderer::embedded::EmbeddedRenderer;
use rusvid_lib::renderer::Renderer;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{ItemStatus, SharedData, SharedItemList};

pub async fn renderer(mut rx: UnboundedReceiver<SharedData>, shared_list: SharedItemList) {
    while let Some(message) = rx.recv().await {
        println!("{}: {:?}", message.id, message.composition);

        shared_list
            .write()
            .unwrap()
            .list
            .insert(message.id.clone(), ItemStatus::Processing);

        let mut renderer = EmbeddedRenderer::new(format!("{}.mp4", message.id));
        renderer.render(message.composition).unwrap();

        shared_list
            .write()
            .unwrap()
            .list
            .insert(message.id.clone(), ItemStatus::Finish);
    }
}
