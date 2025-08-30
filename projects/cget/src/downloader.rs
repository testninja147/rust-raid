use reqwest::{
    self,
    header::{CONTENT_RANGE, RANGE},
};
// use tokio::stream;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

async fn calculate_content_length(url: String) -> Result<usize, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).header(RANGE, "bytes=0-0").send().await?;
    println!("⛔ Headers: {:?}", response.headers());
    if let Some(content_range) = response.headers().get(CONTENT_RANGE) {
        let cr = content_range.to_str()?;
        println!("⛔Content_range: {}", cr);
        return Ok(cr.parse().unwrap_or(0));
    }
    return Ok(0);
}

async fn get_file(
    url: String,
    range: Option<(u8, u8)>,
    threads: u8,
) -> Result<usize, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut stream = response.bytes_stream();
    let mut downloaded = 0usize;
    let mut file = tokio::fs::File::create("output.docx").await?;
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

pub async fn download(url: String, mut threads: u8) -> Result<(), Box<dyn std::error::Error>> {
    if threads > 32 {
        threads = 32;
    }
    if let Ok(file_size) = calculate_content_length(url.clone()).await {
        println!("file size: {:?}", file_size);
    } else {
        println!("File size is not available, skipping threads.");
        threads = 1;
    }
    println!("Downloading the file from the url:{url}");
    println!("Threads: {threads}");
    get_file(url, None, 1).await;
    Ok(())
}
