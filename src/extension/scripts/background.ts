import Browser from "webextension-polyfill"

Browser.runtime.onMessage.addListener(async (message) => {
    if (message.command == "playlists_download"){
        console.log("RECEIVED")
        const playlistids: string[] = message.playlistids
        await window.fetch("http://localhost:33346/download", {
            method: "POST",
            body: JSON.stringify({
                playlistids: playlistids,
                maxdirectories: 10,
                musicdirectory: "X:/Some Useful Stuff/Music/NewStream",
            })
        })
    }
})