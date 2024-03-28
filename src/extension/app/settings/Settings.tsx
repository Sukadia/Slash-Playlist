export default function Settings() {

  return (
    <main className="flex flex-col gap-1 p-2 select-none">
      <div className="flex flex-col gap-2 bg-slate-700 rounded-lg overflow-clip p-2">
        <div>Download Directory:</div>
        <input className="text-black bg-white w-full h-full rounded-sm"/>
      </div>
    </main>
  )
}