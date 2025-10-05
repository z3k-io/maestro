const themes = ["light", "dark"];

interface ThemePickerProps {
  theme: string;
  setTheme: (theme: string) => void;
}

export default function ThemePicker({ theme, setTheme }: ThemePickerProps) {
  return (
    <div className="dropdown dropdown-top flex flex-row gap-4 p-1">
      <h1 className="text-sm content-center">Theme</h1>
      <div tabIndex={0} role="button" className="btn m-1">
        {theme}
        <svg
          width="12px"
          height="12px"
          className="inline-block h-2 w-2 fill-current opacity-60"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 2048 2048"
        >
          <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
        </svg>
      </div>
      <ul tabIndex={0} className="dropdown-content bg-base-300 rounded-box z-[1] w-52 p-2 shadow-2xl">
        {themes.map((themeName) => (
          <li key={themeName}>
            <input
              type="radio"
              name="theme-dropdown"
              className="theme-controller btn btn-sm btn-block btn-ghost justify-start"
              aria-label={themeName}
              checked={theme === themeName}
              onChange={() => setTheme(themeName)}
              value={themeName}
            />
          </li>
        ))}
      </ul>
    </div>
  );
}
