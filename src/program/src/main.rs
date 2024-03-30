use rouille::Response;
use std::process::Command;
use serde::Deserialize;
use serde_json::Value;
use std::io::{Read, Seek, SeekFrom, Write};
use std::fs::{self, OpenOptions};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

#[derive(Deserialize)]
struct RequestBody {
    playlistids: Vec<String>,
    config_path: String
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(10).build_global().unwrap();

    rouille::start_server("localhost:33346", move |request| {
        // localhost:33346/download to trigger playlist download
        if request.method() == "POST" && request.url() == "/download" {

            // Read the body of the request
            let mut data = String::new();
            if let Err(e) = request.data().unwrap().read_to_string(&mut data) {
                eprintln!("Failed to read request body: {}", e);
                return Response::text("Internal Server Error").with_status_code(500);
            };

            // Parse the JSON body of the request
            let body: RequestBody = match serde_json::from_str(&data) {
                Ok(body) => body,
                Err(e) => {
                    eprintln!("Failed to parse JSON body: {}", e);
                    return Response::text("Invalid JSON body").with_status_code(400);
                }
            };

            // Get or create music directory & log file
            let musicdirectory = body.config_path;
            fs::create_dir_all(&musicdirectory).expect("Failed to get Music directory");
            let mut logfile = OpenOptions::new().write(true).read(true).create(true).open(format!("{}/{}",&musicdirectory,".log.json")).expect("Failed to open log file");
            let logtext = &mut String::new();
            logfile.read_to_string(logtext).expect("Failed to read log file");
            let logdatavalue: Value = match serde_json::from_str(&logtext) {
                Ok(value) => value,
                Err(_) => serde_json::json!({})
            };
            let logdata = logdatavalue.as_object().unwrap();
            let newlogdata = Arc::new(Mutex::new(logdata.clone()));

            // Iterate over each URL and execute yt-dlp command
            for playlistid in body.playlistids.iter() {
                let playlistlink = format!("https://www.youtube.com/playlist?list={}", playlistid);

                // Fetch playlist data
                println!("Fetching [{}]...", playlistlink);
                let output = Command::new("yt-dlp")
                    .arg("--flat-playlist")
                    .arg("-J")
                    .arg(&playlistlink)
                    .output()
                    .expect("Failed to run");
                
                let playlistdata: Value = serde_json::from_slice(&output.stdout).unwrap();
                let playlisttitle = playlistdata["title"].as_str().unwrap();
                let playlistentries = playlistdata["entries"].as_array().unwrap();
                println!("Fetched [{}]", playlisttitle);

                // Check for differences between the two playlists
                println!("Checking {} videos...", playlistentries.len());

                let downloadedvalue;
                let value = serde_json::json!([]);
                if logdata.contains_key(playlisttitle) {
                    downloadedvalue = logdata.get(playlisttitle).unwrap();
                } else {
                    downloadedvalue = &value;
                }
                let downloadedids = downloadedvalue.as_array().unwrap();

                // Delete videos no longer in YT playlist
                for videoid in downloadedids.iter() {
                    if !downloadedids.contains(videoid) {
                        // TODO: Find file with metadata and delete, then remove from logdata
                        println!("- Removed {}",videoid);
                    }
                }

                // Download videos that aren't on disk
                playlistentries.into_par_iter().for_each(|entry| {
                    let videoidvalue = &entry["id"];
                    let videoid = videoidvalue.as_str().unwrap();
                    let outputarg = format!("{}{}/{}", &musicdirectory, playlisttitle, "[%(uploader)s] %(title)s.%(ext)s");
                    if !downloadedids.contains(videoidvalue) {
                        let output = Command::new("yt-dlp")
                            .arg("-x")
                            .arg("--audio-format").arg("mp3")
                            .arg("--output").arg(&outputarg)
                            // BUG: Video ID not being placed in file metadata
                            .arg("--parse-metadata").arg(format!("\"{}:%(meta_comment)s\"",&videoid)) 
                            .arg(format!("https://www.youtube.com/watch?v={}",&videoid))
                            .output();

                        if let Ok(output) = output {
                            if output.status.success() {
                                println!("- Downloaded {}",&entry["title"]);
                                let mut dataguard = newlogdata.lock().unwrap();
                                let entry = dataguard.entry(playlisttitle).or_insert_with(|| serde_json::json!([]));
                                entry.as_array_mut().unwrap().push(videoidvalue.clone());
                            } else {
                                println!("- Failed downloading {}",&entry["title"]);
                                eprintln!("{}", String::from_utf8(output.stderr).unwrap());
                            }
                        } else {
                            eprintln!("- Failed to execute yt-dlp command");
                        }
                    } else {
                        println!("- Already downloaded {}",&entry["title"]);
                    }
                });

                println!("Finished [{}]\n", playlisttitle);
            }

            // Write over .log.json with new data
            let newdata = newlogdata.lock().unwrap().to_owned();
            logfile.seek(SeekFrom::Start(0)).expect("Failed to return to beginning of file");
            let _ = logfile.write_all(serde_json::to_string(&newdata).unwrap().as_bytes());

            // Respond with success message
            Response::text("Download started for all URLs")
        } else {
            // Respond with 404 for other routes
            Response::empty_404()
        }
    });
}