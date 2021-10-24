use std::process::Command;
use std::path::Path;
use directories::BaseDirs;

pub fn download(link_ext: &str, link_type: &str, quality: &str, song_id: &str) {
    //youtube-dl.exe
    // --output %localappdata%/GeometryDash/123456789.%(ext)s
    // --extract-audio
    // --audio-quality=128k
    // --audio-format mp3
    // https://www.youtube.com/watch?v=ZmsdIQuywaE

    let mut link = &*format!("https://www.youtube.com/watch?v={}", link_ext);
    Command::new("nong/converter.exe")
        .args(&[
            "--output",
            format!("%localappdata%\\GeometryDash\\{}.%(ext)s", song_id).as_str(),
            "--extract-audio",
            "--audio-quality",
            quality,
            "--audio-format",
            "mp3",
            link
        ])
        .output()
        .expect("failed to execute process");
}

pub fn next_free_id() -> String {
    let mut c = 0;
    loop {
        if let Some(base_dirs) = BaseDirs::new() {
            let path = base_dirs.cache_dir().to_str().unwrap().to_owned() + &*"\\GeometryDash\\".to_owned() + &*c.to_string().to_owned() + &*".mp3".to_owned();
            //println!("{}", path);

            if !Path::new(path.as_str()).exists() {
                //println!("{} exists!", c);
                break;
            }
            c += 1;
        }
    }
    return c.to_string();
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

/*pub fn download(link_ext: &str, link_type: &str, quality: &str, song_id: &str) {
    let content = Rafy::new(&*format!("https://www.youtube.com/watch?v={}", link_ext)).unwrap();
    let audiostreams = content.audiostreams;
    audiostreams[0].download(&*format!("%localappdata%\\GeometryDash\\{}.mp3", song_id));
}*/