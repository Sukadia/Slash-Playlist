import Browser from "webextension-polyfill"
import { loadConfig } from "../app/AppState"

Browser.runtime.onMessage.addListener(async (message) => {
    if (message.command == "playlists_download"){

        const config = await loadConfig()

        const playlistids: string[] = message.playlistids
        await window.fetch("http://localhost:33346/download", {
            method: "POST",
            body: JSON.stringify({
                playlistids: playlistids,
                ...config
            })
        })
        // TODO: Query or open a connection to get status on progress
    }
})