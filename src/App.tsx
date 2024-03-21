import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.scss";

function App() {
  return (
    <div id="window">
      <div className="searchbar-field">
        <input type="text" className="searchbar" />
      </div>
    </div>
  );
}

export default App;
