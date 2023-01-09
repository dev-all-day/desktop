/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        transparent: "transparent",
        current: "currentColor",
        black: colors.black,
        white: colors.white,
        gray: colors.gray,
        emerald: colors.emerald,
        indigo: colors.indigo,
        yellow: colors.yellow,
        orange: colors.orange,
        rose: colors.rose,
        primary: "#1C1E4D",
        secondary: "#FFC629",
        graytext: "#959CAF",
        graybg: "#f1f3f7",
      },
    },
  },
  plugins: [require("tailwind-scrollbar")],
};
