import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";
import { Provider as ReduxProvider } from "react-redux";
import store from "./redux/store";
import { ThemeProvider } from "./components/app";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  // <React.StrictMode>
    <ReduxProvider store={store}>
      <ThemeProvider>
          <App />
      </ThemeProvider>
    </ReduxProvider>
  // </React.StrictMode>
);
