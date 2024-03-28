import React from "react"
import ReactDOM from "react-dom/client"
import "./index.css"
import Settings from "./settings/Settings"

const root = document.createElement("div")
root.className = "container"
document.body.appendChild(root)
const rootDiv = ReactDOM.createRoot(root)

rootDiv.render(
  <React.StrictMode>
    <Settings/>
  </React.StrictMode>
);