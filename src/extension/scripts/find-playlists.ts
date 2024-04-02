import Browser from "webextension-polyfill"

async function checkPage(){
    console.log("SLASH PLAYLIST")

    // TODO: Automate playlist grabbing via https://www.youtube.com/feed/playlists
    // Website update requires webpage to be open to see playlists

    // Find playlists starting with "/"
    const playlists = document.querySelectorAll("[href*=\"&list\"]")
    let downloadplaylists = []
    for (let playlist of playlists){
        const playlistname = playlist.children[0].textContent
        if (playlistname?.indexOf("/") == 0){
            console.log(playlistname)
            const playlistlink = playlist.getAttribute("href")!
            downloadplaylists.push(playlistlink.split("&list=")[1].split("&")[0])
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