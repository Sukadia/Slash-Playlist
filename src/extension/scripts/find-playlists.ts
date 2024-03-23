import Browser from "webextension-polyfill"

async function checkPage(){
    console.log("SLASH PLAYLIST")
    const playlists = document.querySelectorAll("[href*=\"/playlist\"]")
    let downloadplaylists = []
    for (let playlist of playlists){
        const playlistname = playlist.getAttribute("title")
        if (playlistname?.indexOf("/") != -1){
            console.log(playlistname)
            const playlistlink = playlist.getAttribute("href")!
            downloadplaylists.push(playlistlink.split("=")[1])
        }
    }
    Browser.runtime.sendMessage({
        command: "playlists_found",
        ids: downloadplaylists
    })
    console.log(downloadplaylists)
}

Browser.runtime.onMessage.addListener((message) => {
    if (message.command == "playlists_check"){
        checkPage()
    }
})