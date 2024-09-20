/** @type {import('tailwindcss').Config} */
export const darkMode = ["class"];
export const content = ["./pages/**/*.{ts,tsx}", "./components/**/*.{ts,tsx}", "./app/**/*.{ts,tsx}", "./src/**/*.{ts,tsx}"];
export const plugins = [require("tailwindcss-animate"), require("daisyui")];
export const daisyui = {
  themes: ["light", "dark", "synthwave", "cyberpunk", "aqua", "coffee", "night"],
};
