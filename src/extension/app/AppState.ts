import { create } from "zustand"

export interface AppState {
    status: string,
    action: string,
    playlistids: string[]
}

const useAppState = create<AppState>((set) => ({
    status: "not pissed",
    action: "Fetch",
    playlistids: [],
}))

export { useAppState }