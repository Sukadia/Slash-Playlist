use rouille::Response;
use serde::Deserialize;
use std::io::Read;

mod download;

#[derive(Deserialize)]
pub struct RequestBody {
    playlistids: Vec<String>,
    config_path: String
}

pub fn start(){
    println!("Now listening on localhost:33346..");

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

            // Start download of playlists
            download::start(body);

            // Respond with success message
            Response::text("Download started for all URLs")
        } else {
            // Respond with 404 for other routes
            Response::empty_404()
        }
    });
}