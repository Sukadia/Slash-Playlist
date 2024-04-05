mod prereq;

pub fn check_prerequisites(){
    prereq::check_version("yt-dlp".to_string(), Vec::from(["--version"]), None);
    prereq::check_version("ffmpeg".to_string(), Vec::from(["-version"]),
        Some(&|s: String| {let array: Vec<&str> = s.split(" ").collect();
        return array[2].to_string();
    }));
}