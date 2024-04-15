import { FormEvent } from "react";
import "../scss/SearchBar.scss";
import { listen } from "@tauri-apps/api/event";

export default function SearchBar() {
  async function handleInput(e: FormEvent<HTMLInputElement>) {
    //@ts-ignore
    invoke("search_input", { value: e.target.value });
  }

  listen("open_window", (_) => {
    document.getElementById("text-input")?.focus();
  });

  return (
    <div className="searchbar-field">
      <div className="searchbar-box">
        <input type="text" className="searchbar" id="text-input" onInput={handleInput} />
      </div>
    </div>
  );
}
