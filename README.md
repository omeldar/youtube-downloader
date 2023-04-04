# Rust Youtube Downloader

This app is using the ![rusty_ytdl](https://github.com/Mithronn/rusty_ytdl) crate.

## Use

There are a few arguments that can be specified in order to specify download options further

```rust
"--lowest-quality" => video_quality = VideoQuality::Lowest,
"--highest-video" => video_quality = VideoQuality::HighestVideo,
"--audio-only" => video_search_opt = VideoSearchOptions::Audio,
```

