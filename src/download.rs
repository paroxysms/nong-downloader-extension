use reqwest::Error;
use std::process::Command;

pub async fn download(link_ext: &str, link_type: &str, quality: &str, song_id: &str) {

    //youtube-dl.exe
    // --output %localappdata%/GeometryDash/123456789.%(ext)s
    // --extract-audio
    // --audio-quality=128k
    // --audio-format mp3
    // https://www.youtube.com/watch?v=TxP8Wssz-W4

    let mut link = &*format!("https://www.youtube.com/watch?v={}", link_ext);
    Command::new("converter.exe")
        .args(&["youtube-dl.exe",
            format!("--output %localappdata%/GeometryDash/{}.%(ext)s", song_id).as_str(),
            "--extract-audio",
            format!("--audio-quality={}]", quality).as_str(),
            "--audio-format mp3",
            link])
        .output()
        .expect("failed to execute process");
}