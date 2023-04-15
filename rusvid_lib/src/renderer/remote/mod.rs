use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use log::info;
use multipart::client::lazy::Multipart;
use rusvid_core::server::{ItemStatus, ItemStatusResponse};
use ureq::AgentBuilder;
use url::Url;

use crate::composition::Composition;
use crate::renderer::Renderer;

#[derive(Debug)]
pub struct RemoteRenderer {
    id: Option<String>,

    server_uri: Url,

    out_path: PathBuf,
}

impl RemoteRenderer {
    pub fn new(out_path: impl Into<PathBuf>, server_uri: impl Into<String>) -> Result<Self> {
        let url: String = server_uri.into();

        Ok(RemoteRenderer {
            id: None,
            server_uri: Url::parse(&url)?,
            out_path: out_path.into(),
        })
    }
}

impl Renderer for RemoteRenderer {
    fn render(&mut self, composition: Composition) -> Result<()> {
        info!("Using renderer: {:?}", self);

        let agent = AgentBuilder::new().build();

        let yaml_string = serde_yaml::to_string(&composition)?;

        let mut m = Multipart::new();
        m.add_stream(
            "file",
            yaml_string.as_bytes(),
            Some("composition.yml"),
            "text/yaml".parse().ok(),
        );

        let mdata = m.prepare()?;

        let res = agent
            .post(self.server_uri.join("/video/upload")?.as_str())
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary={}", mdata.boundary()),
            )
            .send(mdata)?;

        if res.status() != 201 {
            bail!("Error in uploading composition to server");
        }

        self.id = res.header("x-video-id").map(|h| h.to_string());

        if self.id.is_none() {
            bail!("Error in uploading composition to server");
        }

        // Wait until the video is finish
        while {
            let url = self
                .server_uri
                .join("/status/id/")?
                .join(&self.id.clone().unwrap())?;
            let response: ItemStatusResponse = agent.get(url.as_str()).call()?.into_json()?;
            let status = response.status();

            match status {
                ItemStatus::InDeletion => bail!("Composition is in deletion"),
                ItemStatus::EncounteredError => bail!("Encountered an error with the composition"),
                status => status != ItemStatus::Finish,
            }
        } {
            thread::sleep(Duration::from_millis(750));
        }

        let url = self
            .server_uri
            .join("/video/id/")?
            .join(&self.id.clone().unwrap())?;

        let res = agent.get(url.as_str()).call()?;

        // TODO don't store file in memory, write it somehow directly to the file
        let mut reader = res.into_reader();
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut file = File::create(&self.out_path)?;
        file.write_all(&buffer)?;

        Ok(())
    }

    fn out_path(&self) -> &std::path::Path {
        todo!()
    }

    fn tmp_dir_path(&self) -> &std::path::Path {
        todo!()
    }
}
