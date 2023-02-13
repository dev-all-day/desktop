import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";
import { ThemeContext } from "./contexts";
import { Provider as ReduxProvider } from "react-redux";
import store from "./redux/store";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  // <React.StrictMode>
    <ReduxProvider store={store}>
      <ThemeContext.Provider value="dark">
        <App />
      </ThemeContext.Provider>
    </ReduxProvider>
  // </React.StrictMode>
);
