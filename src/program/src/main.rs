mod webserver;
mod setup;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("Slash Playlist v{}", VERSION);
    setup::check_prerequisites();
    webserver::start();
}