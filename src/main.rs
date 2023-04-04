use text_io::read;
use rusty_ytdl::*;

#[tokio::main]
async fn main() {
    println!("Enter your youtube video url");
    let url: String = read!();
    println!("Enter the target path. '../dir/filename.mp4'");
    let mut target_dir: String = read!();
    if target_dir.is_empty() {
        target_dir = String::from(".mp4");
    }
    println!("Video {} will be saved as {}", url, target_dir);
    
    let video_options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAuido,
        ..Default::default()
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    println!("Downloading data...");
    let video_download_buffer: Result<Vec<u8>, VideoError> = video.download().await;
    println!("Converting to file");
    save_buffer_to_file(target_dir, video_download_buffer);
    println!("Download to file finished.");
}

fn save_buffer_to_file(path: String, buffer: Result<Vec<u8>, VideoError>) -> bool {
    use std::io::Write;

    if buffer.is_ok() {
        let path = std::path::Path::new(&path);
        let mut file = std::fs::File::create(path).unwrap();
        _ = file.write_all(&buffer.unwrap());
    }
    return true;
}
