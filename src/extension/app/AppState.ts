import { create } from "zustand"
import { storage } from "webextension-polyfill"

export interface AppState {
    status: string,
    action: string,
    playlistids: string[],
    config_path: string,
    config_concurrentdownloads: number
}

const useAppState = create<AppState>((set) => ({
    status: "not pissed",
    action: "Fetch",
    playlistids: [],

    config_path: "",
    config_concurrentdownloads: 10,
}))

async function loadConfig(){
    let config = await storage.local.get(["config_path","config_concurrentdownloads"])
    useAppState.setState(config)
    return config
}

loadConfig()

export { useAppState, loadConfig }