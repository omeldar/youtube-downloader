use text_io::read;
use rusty_ytdl::*;

#[tokio::main]
async fn main() {
    println!("Enter your youtube video url");
    let url: String = read!();
    println!("You entered: {}", url);
    
    let video_options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAuido,
        ..Default::default()
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    println!("Downloading...");
    let a: Result<Vec<u8>, VideoError> = video.download().await;
    println!("Error while downloading: {:?}", a.err());
    println!("Download ended.");
}
