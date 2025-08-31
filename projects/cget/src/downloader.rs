use reqwest::{
    self, Url,
    header::{CONTENT_DISPOSITION, CONTENT_RANGE, HeaderMap, RANGE},
};
// use tokio::stream;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

pub(crate) struct Downloader {
    url: String,
    headers: HeaderMap,
    // timeouts, follow_redirects, etc.
    // threads: u8,
}

pub trait HeaderUtils {
    /// # Extract file name
    /// When response header provides content disposition or any other keys to provide
    /// file name or file type, we can extract it from here. We can also guess the
    /// file name from the url and content-type too.
    fn extract_filename(&self) -> Result<String, Box<dyn std::error::Error>>;

    /// # Extract file Size
    /// When response header provides content-range, it is easy to extract the
    /// actual file size in bytes.
    ///
    /// example response: `Content-Range` `bytes 0-0/360996864`
    ///
    /// From the above response header, we can extract value in bytes
    fn extract_file_size(&self) -> Result<usize, Box<dyn std::error::Error>>;
}

impl HeaderUtils for HeaderMap {
    fn extract_filename(&self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(disposition) = &self.get(CONTENT_DISPOSITION) {
            let value = disposition.to_str()?;
            if let Some(filename) = value.split("filename=").nth(1) {
                return Ok(filename.trim_matches('"').to_string());
            }
        }
        return Err(Box::from("Unable to extract filename".to_owned()));
        // TODO: guess filename from content type
    }

    fn extract_file_size(&self) -> Result<usize, Box<dyn std::error::Error>> {
        if let Some(cr) = &self.get(CONTENT_RANGE) {
            if let Some(content_range) = cr.to_str()?.split("/").into_iter().last() {
                return Ok(content_range.parse().unwrap_or(0));
            }
        }
        return Ok(0);
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
            // threads,
            headers: HeaderMap::new(),
        }
    }

    // fixme: use this instead of get_file while handling threads
    async fn get_chunk(
        &self,
        range: Option<(usize, usize)>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.url).await?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0usize;
        println!("");
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            // FIXME: save file TO THE FORMAT PROVIDED IN THE RESPONSE
            // file.write_all(&chunk).await?;
            downloaded += chunk.len();
            print!("\rdownloaded: {:?} bytes", downloaded);
        }
        println!("");
        return Ok(0);
    }

    pub async fn download(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

        let file_size = (&self.headers.extract_file_size().unwrap_or(0)).to_owned();
        println!("⛔file size: {file_size}");

        // todo: handle threads
        let _ = get_file(&self.url, filename).await;
        Ok(())
    }
}

async fn get_file(url: &str, filename: String) -> Result<usize, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = tokio::fs::File::create(filename).await?;
    let mut stream = response.bytes_stream();
    let mut downloaded = 0usize;
    println!("");
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        // FIXME: save file TO THE FORMAT PROVIDED IN THE RESPONSE
        file.write_all(&chunk).await?;
        downloaded += chunk.len();
        print!("\rdownloaded: {:?} bytes", downloaded);
    }
    println!("");
    return Ok(0);
}
