
<p align="center">
    <img src="./src/extension/icons/Icon.svg" alt="Slash Playlist Icon" width="128rem"/>
</p>

# Slash Playlist
A web extension and program using yt-dlp to download YT playlists and sort them into directories based on name.

This repo is split between two projects:

- `src/extension` - A Node.js React web extension, built with webpack.
- `src/program` - A rust program, used as a local webserver to run yt-dlp.

## Usage
The web extension will fetch all playlists from Youtube that have a title starting with a "/" (ex: "/Mood/Jamming") and send it to the program to download into directories.

0. Have [ffmpeg](https://ffmpeg.org/) and [yt-dlp](https://github.com/yt-dlp/yt-dlp) installed on your PATH.
1. Download the Chrome/Firefox extension and executable [here](https://github.com/Sukadia/Slash-Playlist/releases/latest).
2. Install the web extension. Open it, and paste in your Download Directory path. (ex: "X:\Some Useful Stuff\Music\Stream")
3. Run `slash-playlist.exe`.
4. Navigate to Youtube and click "Show More" on the sidebar. Press Fetch and Download in the extension.
5. Monitor the `slash-playlist.exe` prompt to see download progress.

## Development
Use `npm run extension-build` and `npm run program-build` to build each project.