import React from "react";
import { FaSun, FaMoon } from "react-icons/fa";
import { ThemeContext } from "../app/ThemeContext";

const ThemeToggle = () => {
  const { theme, setTheme }: any = React.useContext(ThemeContext);

  return (
    <div className="transition duration-500 ease-in-out rounded-full p-2">
      {theme === "dark" ? (
        <FaSun
          onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          className="text-gray-500 dark:text-gray-400 text-1xl cursor-pointer hover:text-primary"
        />
      ) : (
        <FaMoon
          onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          className="text-gray-500 dark:text-gray-400 text-1xl cursor-pointer hover:text-secondary"
        />
      )}
    </div>
  );
};

export default ThemeToggle;
