import { FormEvent } from "react";
import "../scss/SearchBar.scss";

export default function SearchBar() {
  async function handleInput(e: FormEvent<HTMLInputElement>) {
    //@ts-ignore
    invoke("search_input", { value: e.target.value });
  }

  return (
    <div className="searchbar-field">
      <div className="searchbar-box">
        <input type="text" className="searchbar" onInput={handleInput} />
      </div>
    </div>
  );
}
