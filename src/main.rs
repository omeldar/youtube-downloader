use text_io::read;
use rusty_ytdl::*;
use std::env;

#[tokio::main]
async fn main() {
    println!("Enter your youtube video url");
    let url: String = read!();

    println!("\nEnter the target path. 'D:\\..\\dir\\filename.mp4'");
    let target_path: String = read!();

    println!("\nVideo {} will be saved as {}", url, target_path);

    let args: Vec<String> = env::args().collect();

    let mut video_quality: VideoQuality = VideoQuality::Highest;
    let mut video_search_opt: VideoSearchOptions= VideoSearchOptions::VideoAuido;

    for i in 1..args.len() {
        match args[i].as_str() {
            "--lowest-quality" => video_quality = VideoQuality::Lowest,
            "--highest-video" => video_quality = VideoQuality::HighestVideo,
            "--only-audio" => video_search_opt = VideoSearchOptions::Audio,
            _ => println!("Argument {} not found", args[i])
        }
    }
    
    let video_options = VideoOptions {
        quality: video_quality,
        filter: video_search_opt,
        ..Default::default()
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    println!("Downloading data...");
    let video_download_buffer: Result<Vec<u8>, VideoError> = video.download().await;
    println!("Converting data to file...");
    save_buffer_to_file(target_path, video_download_buffer);
    println!("Download to file completed.");
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
