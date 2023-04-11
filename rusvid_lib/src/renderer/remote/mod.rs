use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use log::info;
use reqwest::Url;
use rusvid_core::server::{ItemStatus, ItemStatusResponse};
use tempfile::NamedTempFile;

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

        let yaml_file = NamedTempFile::new()?;
        serde_yaml::to_writer(&yaml_file, &composition)?;

        let client = reqwest::blocking::Client::new();

        let file_part =
            reqwest::blocking::multipart::Part::file(yaml_file.path())?.mime_str("text/x-yaml")?;
        let form = reqwest::blocking::multipart::Form::new().part("file", file_part);

        let res = client
            .post(self.server_uri.join("/video/upload")?)
            .multipart(form)
            .send()?;

        if res.status() != 201 {
            bail!("Error in uploading composition to server");
        }

        self.id = res
            .headers()
            .get("x-video-id")
            .map(|h| h.to_str().map(|hs| hs.to_string()))
            .transpose()?;

        if self.id.is_none() {
            bail!("Error in uploading composition to server");
        }

        // Wait until the video is finish
        while {
            let res = client
                .get(
                    self.server_uri
                        .join("/status/id/")?
                        .join(&self.id.clone().unwrap())?,
                )
                .send()?;

            let body: ItemStatusResponse = res.json()?;
            let status = body.status();

            if status.is_not_ok() {
                bail!("Unhandled error: {:?}", status)
            }

            status != ItemStatus::Finish
        } {
            thread::sleep(Duration::from_millis(750));
        }

        let res = client
            .get(
                self.server_uri
                    .join("/video/id/")?
                    .join(&self.id.clone().unwrap())?,
            )
            .send()?;

        let bytes = res.bytes()?;
        let mut file = File::create(&self.out_path)?;
        file.write_all(&bytes)?;

        Ok(())
    }

    fn out_path(&self) -> &std::path::Path {
        todo!()
    }

    fn tmp_dir_path(&self) -> &std::path::Path {
        todo!()
    }
}
