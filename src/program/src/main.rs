use rouille::Response;
use std::process::Command;
use serde::Deserialize;
use serde_json::Value;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::fs::{self, OpenOptions};

#[derive(Deserialize)]
struct RequestBody {
    playlistids: Vec<String>,
    maxdirectories: i32,
    musicdirectory: String
}

/* Flat playlist json
{"id": "PLIF2opf2-1Pr0NEsp3YeRlmYVUCXip-CM", "title": "HighAction/Preparing for War", "availability": "unlisted", "channel_follower_count": null, "description": "", "tags": [], "thumbnails": [{"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEWCKgBEF5IWvKriqkDCQgBFQAAiEIYAQ==&rs=AOn4CLAjFlo4hfbnQYB-0Dj4hcpCytqV6g", "height": 94, "width": 168, "id": "0", "resolution": "168x94"}, {"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEWCMQBEG5IWvKriqkDCQgBFQAAiEIYAQ==&rs=AOn4CLCKz-m2-f0jmAoM3zz1qFgcHeU1Vg", "height": 110, "width": 196, "id": "1", "resolution": "196x110"}, {"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEXCPYBEIoBSFryq4qpAwkIARUAAIhCGAE=&rs=AOn4CLDDGuBe8CYKWkiXJM8NuNNTXHDYNQ", "height": 138, "width": 246, "id": "2", "resolution": "246x138"}, {"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEXCNACELwBSFryq4qpAwkIARUAAIhCGAE=&rs=AOn4CLB34oHpsyJHtSRswEaQyuIU2rCDQQ", "height": 188, "width": 336, "id": "3", "resolution": "336x188"}], "modified_date": "20240318", "view_count": 5, "playlist_count": 4, "channel": "Sukadia", "channel_id": "UC0Xy-rOR0lzrEm9Is87LZQQ", "uploader_id": "@Sukadia", "uploader": "Sukadia", "channel_url": "https://www.youtube.com/channel/UC0Xy-rOR0lzrEm9Is87LZQQ", "uploader_url": "https://www.youtube.com/@Sukadia", "_type": "playlist", "entries": [{"_type": "url", "ie_key": "Youtube", "id": "Je8iIWgavh4", "url": "https://www.youtube.com/watch?v=Je8iIWgavh4", "title": "Lost Odyssey OST - Disc1 - Track16 - March to War", "description": null, "duration": 115, "channel_id": "UCQtGAW6DyhZOzblV7XPnVLA", "channel": "Synapsidae", "channel_url": "https://www.youtube.com/channel/UCQtGAW6DyhZOzblV7XPnVLA", "uploader": "Synapsidae", "uploader_id": "@Synapsidae", "uploader_url": "https://www.youtube.com/@Synapsidae", "thumbnails": [{"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVfKriqkDDggBFQAAiEIYAXABwAEG&rs=AOn4CLBfQG5n_K6w0CILkXHHCC1FppBVNg", "height": 94, "width": 168}, {"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVfKriqkDDggBFQAAiEIYAXABwAEG&rs=AOn4CLDYvQUXfR06J9-WM13lXdjavtAnXQ", "height": 110, "width": 196}, {"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBvjdr3xY3ED07As8Au397wAF_AhQ", "height": 138, "width": 246}, {"url": "https://i.ytimg.com/vi/Je8iIWgavh4/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCwmic63fflAG90E5_xQKZX2SYL5Q", "height": 188, "width": 336}], "timestamp": null, "release_timestamp": null, "availability": null, "view_count": 51000, "live_status": null, "channel_is_verified": null, "__x_forwarded_for_ip": null}, {"_type": "url", "ie_key": "Youtube", "id": "nvQNUxoN8gw", "url": "https://www.youtube.com/watch?v=nvQNUxoN8gw", "title": "Lost Odyssey OST - Disc2 - Track15 - Large Cruise", "description": null, "duration": 180, "channel_id": "UCQtGAW6DyhZOzblV7XPnVLA", "channel": "Synapsidae", "channel_url": "https://www.youtube.com/channel/UCQtGAW6DyhZOzblV7XPnVLA", "uploader": "Synapsidae", "uploader_id": "@Synapsidae", "uploader_url": "https://www.youtube.com/@Synapsidae", "thumbnails": [{"url": "https://i.ytimg.com/vi/nvQNUxoN8gw/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVfKriqkDDggBFQAAiEIYAXABwAEG&rs=AOn4CLBnBUFPEGqjo8U8_8vyisuE3B32YA", "height": 94, "width": 168}, {"url": "https://i.ytimg.com/vi/nvQNUxoN8gw/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVfKriqkDDggBFQAAiEIYAXABwAEG&rs=AOn4CLD8BPiUYuw28-cKxrxPnrW7IMON4g", "height": 110, "width": 196}, {"url": "https://i.ytimg.com/vi/nvQNUxoN8gw/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDRpBq8tooITqxy6DR6Mrgu7OFTRw", "height": 138, "width": 246}, {"url": "https://i.ytimg.com/vi/nvQNUxoN8gw/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLC3ET6vh59g9CyOCmsphXAMMqUjMw", "height": 188, "width": 336}], "timestamp": null, "release_timestamp": null, "availability": null, "view_count": 23000, "live_status": null, "channel_is_verified": null, "__x_forwarded_for_ip": null}, {"_type": "url", "ie_key": "Youtube", "id": "ZxnMdkQ-OXE", "url": "https://www.youtube.com/watch?v=ZxnMdkQ-OXE", "title": "Tournament Grid - Super Smash Bros. Brawl", "description": null, "duration": 83, "channel_id": "UC6PshCd-mkeSGNfNO0yWlGg", "channel": "GilvaSunner: Archive", "channel_url": "https://www.youtube.com/channel/UC6PshCd-mkeSGNfNO0yWlGg", "uploader": "GilvaSunner: Archive", "uploader_id": "@GilvaSunnerArchiver", "uploader_url": "https://www.youtube.com/@GilvaSunnerArchiver", "thumbnails": [{"url": "https://i.ytimg.com/vi/ZxnMdkQ-OXE/hqdefault.jpg?sqp=-oaymwE1CKgBEF5IVfKriqkDKAgBFQAAiEIYAXABwAEG8AEB-AH-CYAC0AWKAgwIABABGCcgMCh_MA8=&rs=AOn4CLARbRYO9yzYqfEvCJdbsNb6JB_uoA", "height": 94, "width": 168}, {"url": "https://i.ytimg.com/vi/ZxnMdkQ-OXE/hqdefault.jpg?sqp=-oaymwE1CMQBEG5IVfKriqkDKAgBFQAAiEIYAXABwAEG8AEB-AH-CYAC0AWKAgwIABABGCcgMCh_MA8=&rs=AOn4CLAFhh_lVPHBM6Y1TKDp8wq5U8CUpQ", "height": 110, "width": 196}, {"url": "https://i.ytimg.com/vi/ZxnMdkQ-OXE/hqdefault.jpg?sqp=-oaymwE2CPYBEIoBSFXyq4qpAygIARUAAIhCGAFwAcABBvABAfgB_gmAAtAFigIMCAAQARgnIDAofzAP&rs=AOn4CLB_CGKFPnroPbYrmBsSaEhEJZVelw", "height": 138, "width": 246}, {"url": "https://i.ytimg.com/vi/ZxnMdkQ-OXE/hqdefault.jpg?sqp=-oaymwE2CNACELwBSFXyq4qpAygIARUAAIhCGAFwAcABBvABAfgB_gmAAtAFigIMCAAQARgnIDAofzAP&rs=AOn4CLCErkQjnIxMhuIAzCZF0wNR0dirQw", "height": 188, "width": 336}], "timestamp": null, "release_timestamp": null, "availability": null, "view_count": 1000, "live_status": null, "channel_is_verified": null, "__x_forwarded_for_ip": null}, {"_type": "url", "ie_key": "Youtube", "id": "M0_Zvp59TaA", "url": "https://www.youtube.com/watch?v=M0_Zvp59TaA", "title": "Extraction Theme | Full High Quality Extraction and Mission Complete Music | Helldivers 2 OST", "description": null, "duration": 297, "channel_id": "UCPAywPMQRGhGXMUU84BjiAA", "channel": "Mootacoo", "channel_url": "https://www.youtube.com/channel/UCPAywPMQRGhGXMUU84BjiAA", "uploader": "Mootacoo", "uploader_id": "@Mootacoo", "uploader_url": "https://www.youtube.com/@Mootacoo", "thumbnails": [{"url": "https://i.ytimg.com/vi/M0_Zvp59TaA/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVfKriqkDDggBFQAAiEIYAXABwAEG&rs=AOn4CLDTFRXPpYg55IEb-oCJI6nBos2k7g", "height": 94, "width": 168}, {"url": "https://i.ytimg.com/vi/M0_Zvp59TaA/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVfKriqkDDggBFQAAiEIYAXABwAEG&rs=AOn4CLANdMbJnSqJuRXKdgONe5EwCF-QBg", "height": 110, "width": 196}, {"url": "https://i.ytimg.com/vi/M0_Zvp59TaA/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLB-ynea82Ck44iyYBEHMpJj9CWEPg", "height": 138, "width": 246}, {"url": "https://i.ytimg.com/vi/M0_Zvp59TaA/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBncD45GylIf3zNMYwDDupAJ3rvcg", "height": 188, "width": 336}], "timestamp": null, "release_timestamp": null, "availability": null, "view_count": 50000, "live_status": null, "channel_is_verified": null, "__x_forwarded_for_ip": null}], "extractor_key": "YoutubeTab", "extractor": "youtube:tab", "webpage_url": "https://www.youtube.com/playlist?list=PLIF2opf2-1Pr0NEsp3YeRlmYVUCXip-CM", "original_url": "https://www.youtube.com/playlist?list=PLIF2opf2-1Pr0NEsp3YeRlmYVUCXip-CM", "webpage_url_basename": "playlist", "webpage_url_domain": "youtube.com", "release_year": null, "epoch": 1711076925, "__files_to_move": {}, "_version": {"version": "2024.03.10", "current_git_head": null, "release_git_head": "615a84447e8322720be77a0e64298d7f42848693", "repository": "yt-dlp/yt-dlp"}}
*/

fn main() {

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
            let musicdirectory = body.musicdirectory;
            fs::create_dir_all(&musicdirectory).expect("Failed to get Music directory");
            let mut logfile = OpenOptions::new().write(true).read(true).create(true).open(format!("{}{}",&musicdirectory,"/.log.json")).expect("Failed to open log file");
            let logtext = &mut String::new();
            logfile.read_to_string(logtext).expect("Failed to read log file");
            let logdatavalue: Value = match serde_json::from_str(&logtext) {
                Ok(value) => value,
                Err(_) => serde_json::json!({})
            };
            let logdata = logdatavalue.as_object().unwrap();

            // Construct yt-dlp arguments
            let mut parsemetadata_arg = format!("%(playlist)s:(?P<directory1>[^/]+)");
            let mut output_arg = format!("{}/%(directory1)s/", &musicdirectory);
            for i in 2..=body.maxdirectories {
                parsemetadata_arg += &format!("(?:/(?P<directory{}>[^/]+))?", i);
                output_arg += &format!("%(directory{}|)s/", i);
            }
            output_arg += "[%(uploader)s] %(title)s.%(ext)s";

            // Iterate over each URL and execute yt-dlp command
            for playlistid in body.playlistids.iter() {
                let playlistlink = format!("https://www.youtube.com/playlist?list={}", playlistid);

                // Fetch playlist data
                eprintln!("Fetching [{}]...", playlistlink);
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
                if logdata.contains_key(&musicdirectory) {
                    downloadedvalue = logdata.get(&musicdirectory).unwrap();
                } else {
                    downloadedvalue = &value;
                }
                let downloadedids = downloadedvalue.as_array().unwrap();

                // Delete videos no longer in YT playlist
                for videoid in downloadedids.iter() {
                    if !downloadedids.contains(videoid) {
                        // TODO: find file with metadata and delete, then remove from logdata
                        eprintln!("- Removed {}",videoid);
                    }
                }

                // Download videos that aren't on disk
                for entry in playlistentries.iter() {
                    let videoid = &entry["id"];
                    if !downloadedids.contains(&videoid) {
                        // TODO: download video file, add to logdata
                        eprintln!("- Downloaded {}",&entry["title"]);
                    }
                }

                // TODO: write over .log.json with new data

                println!("Finished [{}]", playlisttitle);


                // // Execute yt-dlp command for the URL
                // let output = Command::new("yt-dlp")
                //     .arg("-x")
                //     .arg("-f").arg("\"bestaudio\"")
                //     .arg("--audio-format").arg("mp3")
                //     .arg("--parse-metadata").arg(&parsemetadata_arg)
                //     .arg("--output").arg(&output_arg)
                //     .arg("--parse-metadata").arg("\"%(id)s:%(meta_comment)s\"")
                //     //.arg("--extractor-args youtube:skip=translated_subs;youtubetab:skip=webpage;youtube:skip=hls,dash")
                //     .arg(&playlistlink)
                //     .output();

                // // Check if command execution was successful
                // if let Ok(output) = output {
                //     if output.status.success() {
                //         println!("Downloaded {} successfully", playlistid);
                //     } else {
                //         eprintln!("Failed to download {}", String::from_utf8(output.stderr).unwrap());
                //     }
                // } else {
                //     eprintln!("Failed to execute yt-dlp command");
                // }
            }

            // Respond with success message
            Response::text("Download started for all URLs")
        } else {
            // Respond with 404 for other routes
            Response::empty_404()
        }
    });
}

