import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { relaunch } from "@tauri-apps/api/process";
import { open } from "@tauri-apps/api/shell";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [ip, setIP] = useState("");
  const [name, setName] = useState("");

  async function greet(e: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: e }));
  }

  async function shout(e: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("shout", { phrase: e }));
  }

  const inpRef: any = React.useRef(null);

  const [events, setEvents] = useState<any>([]);

  React.useEffect(() => {
    if (name) shout(name);
    else setGreetMsg("");

    inpRef.current.focus();
  }, [name]);

  async function getIP() {
    setIP(await invoke("my_ip"));
  }

  React.useEffect(() => {
    getIP();
  }, []);

  useEffect(() => {
    if ("EventSource" in window) {
      const eventSource = new EventSource("http://127.0.0.1:9000/events");

      eventSource.onopen = () => {
        // setEvents((events: any) => events.concat("Connection Opened"));
        console.log("Connection Opened");
      };

      eventSource.onmessage = (event) => {
        console.log(JSON.parse(event.data));
        setEvents((events: any) => [...events, event.data]);
      };

      // eventSource.onmessage = (event) => {
      //   const parsedData = JSON.parse(event.data);

      //   setEvents((events:any) => events.concat(parsedData));
      // };

      eventSource.onerror = (event) => {
        console.error(event);
      };

      return () => {
        eventSource.close();
      };
    } else {
      // throw fatal error
    }
  }, []);

  const Logo = () => {
    return (
      <span style={{ display: "flex", justifyContent: "center", alignContent: "center", color: "rgb(104, 149, 242)" }}>
        <span>{"{"}</span>
        <span>dev.all.day</span>
        <span>{"}"}</span>
      </span>
    );
  };

  return (
    <div className="container">
      <h1 style={{ display: "flex", gap: 8, justifyContent: "center" }}>
        <span>Welcome to</span> <Logo />
      </h1>

      <div className="row">
        <div>
          <input
            ref={inpRef}
            id="greet-input"
            value={name}
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
            autoComplete="off"
          />
          <button type="button" onClick={() => setName("")}>
            Clear
          </button>
        </div>
      </div>
      <p>{greetMsg}</p>
      <p>{ip}</p>

      {events.length > 0 ? (
        <ul>
          {events.map((event: any, index: any) => (
            <li key={index}>{event}</li>
          ))}
        </ul>
      ) : null}

      {/* <div style={{ marginTop: 40 }}>
        <button type="button" onClick={async () => await relaunch()}>
          Restart
        </button>
      </div> */}
      <div style={{ marginTop: 40 }}>
        <button type="button" onClick={async () => await open("http://localhost:9000")}>
          Open In Browser
        </button>
      </div>
    </div>
  );
}

export default App;
