import React, { useEffect, useState, useContext } from "react";
import { ThemeContext } from "./contexts";
import { invoke } from "@tauri-apps/api/tauri";
import { relaunch } from "@tauri-apps/api/process";
import { open } from "@tauri-apps/api/shell";
import "./App.css";

import { IoShuffleSharp } from "react-icons/io5";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [ip, setIP] = useState("");
  const [name, setName] = useState("");

  // const addresses: string[] = ["127.0.0.1", "localhost"];

  const [addresses, setAddresses] = useState<string[]>(["localhost", "127.0.0.1"]);
  const [address, setAddress] = useState<any>(addresses[0]);

  const theme = useContext(ThemeContext);

  const shuffleAddresses = () => {
    setAddress((currentAddress: string) => {
      const total = addresses.length;
      const currentIndex = addresses.indexOf(currentAddress);
      if (currentIndex !== -1) {
        if (currentIndex + 1 === total) {
          return addresses[0];
        } else {
          return addresses[currentIndex + 1];
        }
      } else {
        return currentAddress;
      }
    });
  };

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

  React.useEffect(() => {
    if (ip && addresses.length < 3 && ip !== "127.0.0.1") setAddresses((addresses: any) => addresses.concat(ip));
  }, [ip]);

  useEffect(() => {
    if ("EventSource" in window) {
      const eventSource = new EventSource("http://127.0.0.1:3310/events");

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
      <span style={{ color: "rgb(104, 149, 242)" }} className="text-2xl">
        {/* <span style={{ display: "flex", justifyContent: "center", alignContent: "center", color: "rgb(104, 149, 242)" }}> */}
        <span>{"{"}</span>
        <span>dev.all.day</span>
        <span>{"}"}</span>
      </span>
    );
  };

  return (
    <div className="flex flex-col h-screen overflow-hidden bg-graybg ">
      <header className="flex flex-row justify-between items-center w-full h-10 bg-white px-4">
        <Logo />
        <div className="flex gap-4">
          {/* <span>Donate</span> */}
          <span className="cursor-pointer hover:underline" onClick={async () => await open("http://localhost:3310")}>
            Open In Browser
          </span>
        </div>
      </header>
      <main className="flex-grow flex flex-row overflow-hidden justify-center gap-8">
        {/* <main className="flex-1 flex flex-col p-4 justify-center items-center gap-8"> */}

        <div className="flex-shrink-0 w-1/5 p-4 bg-gray-700 flex flex-col gap-4">
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer flex flex-col justify-between group">
            <span></span>
            <hr className="group-hover:border-gray-700" />
            <span></span>
          </div>
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer"></div>
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer"></div>
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer"></div>
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer"></div>
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer"></div>
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer"></div>
          <div className="bg-white rounded-lg w-full h-16 hover:bg-gray-400 cursor-pointer"></div>
        </div>

        <div className="flex-1 flex flex-col p-4">
          <>
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

            {/* <p>{greetMsg}</p> */}

            {events.length > 0 ? (
              <ul>
                {events.map((event: any, index: any) => (
                  <li key={index}>{event}</li>
                ))}
              </ul>
            ) : null}
          </>
        </div>

        <div className="flex-shrink-0 w-1/4 p-4 bg-blue-700">Theme {theme}</div>

        {/* <div style={{ marginTop: 40 }}>
        <button type="button" onClick={async () => await relaunch()}>
          Restart
        </button>
      </div> */}
      </main>
      <footer className="flex flex-row gap-4 w-full h-10 bg-white justify-center items-center">
        <div className="flex flex-row gap-1 items-center">
          <span>Address</span>
          <span className="bg-slate-500 rounded text-gray-200 px-2">{address}</span>
          <IoShuffleSharp className="cursor-pointer" onClick={() => shuffleAddresses()} />
        </div>
        <div className="flex flex-row gap-1">
          <span>Port</span>
          <span className="bg-slate-500 rounded text-gray-200 px-2">3310</span>
        </div>
      </footer>
    </div>
  );
}

export default App;
