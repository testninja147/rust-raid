use std::{cmp::min, error::Error, sync::Arc};

use reqwest::{
    self, Url,
    header::{CONTENT_DISPOSITION, CONTENT_RANGE, HeaderMap, RANGE},
};
// use tokio::stream;
use futures_util::{StreamExt, future};
use indicatif::{HumanBytes, MultiProgress, ProgressBar, ProgressStyle};
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::Mutex;

#[derive(Debug)]
struct Chunk {
    start_byte: u64,
    end_byte: u64,
    downloaded: u64,
}
impl Chunk {
    fn new(start_byte: u64, end_byte: u64) -> Self {
        Self {
            start_byte,
            end_byte,
            downloaded: 0,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Downloader {
    url: String,
    headers: HeaderMap,
    file_size: Option<u64>,
    filename: Option<String>,
    chunks: Arc<Mutex<Vec<Chunk>>>, // this stores downloaded chunk size
}

pub trait HeaderUtils {
    /// # Extract file name
    /// When response header provides content disposition or any other keys to provide
    /// file name or file type, we can extract it from here. We can also guess the
    /// file name from the url and content-type too.
    fn extract_filename(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    /// # Extract file Size
    /// When response header provides content-range, it is easy to extract the
    /// actual file size in bytes.
    ///
    /// example response: `Content-Range` `bytes 0-0/360996864`
    ///
    /// From the above response header, we can extract value in bytes
    fn extract_file_size(&self) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>;
}

impl HeaderUtils for HeaderMap {
    fn extract_filename(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(disposition) = &self.get(CONTENT_DISPOSITION) {
            let value = disposition.to_str()?;
            if let Some(filename) = value.split("filename=").nth(1) {
                return Ok(filename.trim_matches('"').to_string());
            }
        }
        return Err(Box::from("Unable to extract filename".to_owned()));
        // TODO: guess filename from content type
    }

    fn extract_file_size(&self) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let &cr = &self
            .get(CONTENT_RANGE)
            .ok_or_else(|| Box::<dyn Error + Send + Sync>::from("Content_range not found"))?;
        let content_range =
            cr.to_str()?.split("/").into_iter().last().ok_or_else(|| {
                Box::<dyn Error + Send + Sync>::from("Invalid Content_range_format")
            })?;
        Ok(content_range.parse()?)
    }
}

/// # Extract file name from Urls
/// This method is used when we do not have any headers passed for file name
/// For example: if content disposition is not provided, but there is a valid
/// filename in the request url
pub fn extract_filename_from_url(url: &str) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(&url) {
        if let Some(segment) = parsed_url.path_segments().and_then(|s| s.last()) {
            if !segment.is_empty() {
                return Some(segment.to_string());
            }
        }
    }
    return None;
}

impl Downloader {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            headers: HeaderMap::new(),
            file_size: None,
            filename: None,
            chunks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn get_chunk(
        &self,
        range: Option<(u64, u64)>,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut builder = client.get(&self.url);
        if let Some((start, end)) = range {
            builder = builder.header(RANGE, &format!("bytes={start}-{end})"));
        }

        let response = builder.send().await?;
        let mut file = tokio::fs::File::create(self.filename.clone().unwrap()).await?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;
        let bar = match self.file_size {
            Some(size) => {
                let bar = ProgressBar::new(size);
                bar.set_style(ProgressStyle::with_template(
                    "[{elapsed_precise}] {wide_bar:40.white/black} {binary_bytes}/{binary_total_bytes} ({percent}%) {msg}"
                ).unwrap());
                bar
            }
            None => ProgressBar::new_spinner(),
        };

        // progress_bar
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            // FIXME: save file TO THE FORMAT PROVIDED IN THE RESPONSE
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            if let Some(_) = self.file_size {
                bar.inc(chunk.len() as u64);
            } else {
                bar.tick();
                bar.set_message(format!("downloaded: {}", HumanBytes(downloaded),));
            }
        }
        bar.finish();

        return Ok(0);
    }

    pub async fn download(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        // get response headers to get file name, length, etc.
        let response = client
            .get(&self.url)
            .header(RANGE, "bytes=0-0")
            .send()
            .await?;
        self.headers = response.headers().clone().to_owned();

        let filename = match &self.headers.extract_filename() {
            Ok(filename) => filename.to_owned(),
            Err(_) => match extract_filename_from_url(&self.url) {
                Some(filename) => filename,
                None => "download.bin".to_owned(),
            },
        };
        println!("⛔filename: {filename}");
        self.filename = Some(format!("{path}/{filename}"));

        if let Ok(file_size) = self.headers.extract_file_size() {
            self.file_size = Some(file_size);
            println!("⛔file size: {}", HumanBytes(file_size));
        } else {
            println!("⛔ Unable to determine the file size. skipping threads")
        }

        // todo: handle threads
        // let _ = get_file(&self.url, format!("{path}/{filename}")).await;
        let _ = self.get_chunk(None).await;
        Ok(())
    }
}
