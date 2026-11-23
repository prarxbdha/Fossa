import { useEffect, useState, type RefObject } from "react";

interface AddressBarProps {
  url: string;
  onNavigate: (url: string) => void;
  inputRef?: RefObject<HTMLInputElement | null>;
}

function SearchIcon() {
  return (
    <svg className="address-icon" viewBox="0 0 24 24" aria-hidden="true">
      <circle cx="11" cy="11" r="6.5" />
      <path d="m16 16 4 4" />
    </svg>
  );
}

export function AddressBar({ url, onNavigate, inputRef }: AddressBarProps) {
  const [value, setValue] = useState(url);

  useEffect(() => setValue(url), [url]);

  return (
    <form
      className="address-bar"
      role="search"
      onSubmit={(event) => {
        event.preventDefault();
        onNavigate(value.trim());
      }}
    >
      <SearchIcon />
      <input
        ref={inputRef}
        aria-label="Address"
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder="Search or enter address"
        spellCheck={false}
        autoComplete="off"
      />
    </form>
  );
}
