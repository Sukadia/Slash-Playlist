import Browser from "webextension-polyfill"

Browser.runtime.onMessage.addListener(async (message) => {
    if (message.command == "playlists_download"){
        console.log("RECEIVED")
        const playlistids: string[] = message.playlistids
        // TODO: Make body configurable via UI
        await window.fetch("http://localhost:33346/download", {
            method: "POST",
            body: JSON.stringify({
                playlistids: playlistids,
                musicdirectory: "X:/Some Useful Stuff/Music/NewStream",
            })
        })
        // TODO: Query or open a connection to get status on progress
    }
})