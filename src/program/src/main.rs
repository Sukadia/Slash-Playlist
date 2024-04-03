use id3::frame::Comment;
use rouille::Response;
use std::process::Command;
use serde::Deserialize;
use serde_json::Value;
use std::io::{Read, Seek, SeekFrom, Write};
use std::fs::{create_dir_all, read_dir, remove_file, OpenOptions};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use id3::{Content, Frame, Tag, TagLike};

#[derive(Deserialize)]
struct RequestBody {
    playlistids: Vec<String>,
    config_path: String
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(10).build().unwrap();

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

            // TODO: Branch off here and call the rest as a function

            println!("\nSyncing {} playlists...\n",body.playlistids.len());

            // Get or create music directory & log file
            let musicdirectory = body.config_path;
            create_dir_all(&musicdirectory).expect("Failed to get Music directory");
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
                for videoidvalue in downloadedids.iter() {
                    let videoid = videoidvalue.as_str().unwrap();
                    if !playlistentries.iter().any(|entry| entry["id"] == videoid) {
                        let directory = read_dir(format!("{}{}",musicdirectory,playlisttitle));
                        let mut foundfile = false;
                        if let Ok(files) = directory {
                            for file in files {
                                if let Ok(file) = file {
                                    let tags = Tag::read_from_path(file.path());
                                    if let Ok(tags) = tags {
                                        println!("{}",tags.comments().count());
                                        if tags.comments().any(|v| v.text == videoid){
                                            foundfile = true;
                                            println!("- Removed [https://www.youtube.com/watch?v={}]",videoid);
                                            let _ = remove_file(file.path());
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        // Remove videoid from logdata
                        let mut dataguard = newlogdata.lock().unwrap();
                        let entry = dataguard.entry(playlisttitle).or_insert_with(|| serde_json::json!([]));
                        let array = entry.as_array_mut().unwrap();
                        let index = array.iter().position(|id| id.as_str() == Some(videoid)).unwrap();
                        array.swap_remove(index);

                        if !foundfile {
                            println!("- Removed [https://www.youtube.com/watch?v={}] from logfile, couldn't find on disk",videoid);
                        }
                    }
                }

                // Download videos that aren't on disk
                playlistentries.into_par_iter().for_each(|entry| {
                    let videoidvalue = &entry["id"];
                    let videoid = videoidvalue.as_str().unwrap().to_string();
                    let outputarg = format!("{}{}/{}", &musicdirectory, playlisttitle, "[%(uploader)s] %(title)s.%(ext)s");
                    if !downloadedids.contains(videoidvalue) {
                        // Download the video as mp3
                        let output = Command::new("yt-dlp")
                            .arg("--print").arg("filename")
                            .arg("--no-simulate")
                            .arg("-x")
                            .arg("--audio-format").arg("mp3")
                            .arg("--output").arg(&outputarg)
                            .arg("--embed-metadata")
                            .arg(format!("https://www.youtube.com/watch?v={}",&videoid))
                            .output();

                        if let Ok(output) = output {
                            if output.status.success() {
                                let filepath = String::from_utf8(output.stdout).unwrap().replace(".webm\n", ".mp3").to_string();

                                // Write videoid to metadata; required since ffmpeg doesn't do it properly (https://stackoverflow.com/a/61991841)
                                let mut tags = Tag::read_from_path(&filepath).expect("No file found");
                                tags.add_frame(Frame::with_content("COMM", Content::Comment(Comment{
                                    lang: "eng".to_owned(),
                                    description: "videoid".to_owned(),
                                    text: videoid
                                })));
                                tags.write_to_path(&filepath, id3::Version::Id3v24).expect("Couldn't write ID tag");

                                // Add videoid to logdata
                                let mut dataguard = newlogdata.lock().unwrap();
                                let logentry = dataguard.entry(playlisttitle).or_insert_with(|| serde_json::json!([]));
                                logentry.as_array_mut().unwrap().push(videoidvalue.clone());

                                println!("- Downloaded {}",&entry["title"]);
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
            let bytes = serde_json::to_string(&newdata).unwrap();
            logfile.set_len(bytes.len().try_into().unwrap()).expect("Failed to resize log file");
            logfile.seek(SeekFrom::Start(0)).expect("Failed to return to beginning of file");
            logfile.write_all(bytes.as_bytes()).expect("Faield to write to log file");
            
            println!("Finished syncing playlists.");

            // Respond with success message
            Response::text("Download started for all URLs")
        } else {
            // Respond with 404 for other routes
            Response::empty_404()
        }
    });
}