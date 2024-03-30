import { storage } from "webextension-polyfill"
import { useAppState } from "../AppState"
import { useRef } from "react"

export default function Settings() {

  const configPath = useAppState((state) => state.config_path)

  const pathRef = useRef<HTMLInputElement>(null)

  async function handleConfigEnter(e: React.KeyboardEvent<HTMLInputElement>){
    if (e.key == "Enter") {
      let newvalues = {
        config_path: pathRef.current?.value
      }
      useAppState.setState(newvalues)
      storage.local.set(newvalues)
    }
  }


  return (
    <main className="flex flex-col gap-1 p-2 select-none">
      <div className="flex flex-col gap-2 bg-slate-700 rounded-lg overflow-clip p-2">
        <div>Download Directory:</div>
        <input className="text-black bg-white w-full h-full rounded-sm" defaultValue={configPath} onKeyUp={handleConfigEnter} ref={pathRef}/>
      </div>
    </main>
  )
}