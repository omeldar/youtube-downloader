use text_io::read;
use rusty_ytdl::*;

#[tokio::main]
async fn main() {
    println!("Enter your youtube video url");
    let url: String = read!();
    println!("{}", url);

    let video_options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAuido,
        ..Default::default()
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    _ = video.download().await;
}
