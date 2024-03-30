import Browser from "webextension-polyfill"
import { useEffect } from "react"
import { useAppState } from "../AppState"
import { Icon } from "@iconify/react"

export default function Popup() {

  const currentStatus = useAppState((state) => state.status)
  const currentAction = useAppState((state) => state.action)
  const currentPlaylistIds = useAppState((state) => state.playlistids)
  const configPath = useAppState((state) => state.config_path)

  async function handleClick(){
    const tab = await Browser.tabs.query({ active: true, currentWindow: true })
    if (currentAction == "Fetch"){
      Browser.tabs.sendMessage(tab[0].id!,{
        command: "playlists_check"
      })
    }else if (currentAction == "Download"){
      Browser.runtime.sendMessage({
        command: "playlists_download",
        playlistids: currentPlaylistIds
      })
      useAppState.setState({status: `Downloading (0/${currentPlaylistIds.length})`, action: "aight bet"})
    }
  }

  async function handleSettingsClick(){
    await Browser.runtime.openOptionsPage()
  }

  useEffect(() => {
    async function onMessage(message: any){
      if (message.command == "playlists_found"){
        const playlistids: string[] = message.ids
        useAppState.setState({status: `Found ${playlistids.length} playlists`, action: "Download", playlistids: playlistids})
      }
    }
    Browser.runtime.onMessage.addListener(onMessage)
    return () => Browser.runtime.onMessage.removeListener(onMessage)
})

  return (
    <main className="flex flex-col gap-1 p-2 select-none w-40">
      <div className="flex flex-col items-center mx-auto justify-center gap-1 mb-1">
        <img src="../icons/128.png" width={128} height={128} alt="Sukadia logo" className="w-16 h-16"/>
        <div className="flex flex-row text-xs gap-1">
          <div>Status:</div>
          <div className="text-yellow-400">{currentStatus}</div>
        </div>
      </div>
      <div className="flex flex-row mb-1 h-8 w-full">
        <div className={"flex items-center rounded-l-lg w-28 h-8 justify-center " + (configPath ? "cursor-pointer active:bg-slate-500 bg-slate-600 hover:bg-slate-700" : "bg-slate-800")} onClick={configPath ? handleClick : undefined}>{currentAction}</div>
        <div className={"flex w-8 h-8 rounded-r-lg items-center " + (configPath ? "active:bg-slate-700 bg-slate-800 hover:bg-slate-900" : "active:bg-red-600 bg-red-700 hover:bg-red-800")} onClick={handleSettingsClick}>
          <Icon icon={configPath ? "tabler:settings" : "tabler:settings-exclamation"} className="w-7 h-7 mx-auto"/>
        </div>
      </div>
    </main>
  )
}