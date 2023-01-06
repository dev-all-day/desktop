import React, { useEffect, useState, useContext } from "react";
import { ThemeContext } from "./contexts";
import { invoke } from "@tauri-apps/api/tauri";
import { relaunch } from "@tauri-apps/api/process";
import { open } from "@tauri-apps/api/shell";
import "./App.css";

import { IoShuffleSharp } from "react-icons/io5";
// @ts-ignore
import ReactDataViewer from "react-data-viewer";

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

  // const inpRef: any = React.useRef(null);

  const [events, setEvents] = useState<any>([]);

  React.useEffect(() => {
    if (name) shout(name);
    else setGreetMsg("");

    // inpRef.current.focus();
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
    <div className="flex flex-col h-screen overflow-hidden bg-graybg">
      <div className="flex-grow flex flex-row overflow-hidden justify-center">
        <div className="flex-shrink-0 w-72 p-2 bg-[#1e1f21] flex flex-col gap-2 justify-between">
          <div className="text-gray-100 bg-[#131415] rounded-md w-full p-3 cursor-pointer hover:bg-gray-500 hover:text-gray-200">
            Connection #1
          </div>

          <div className="flex flex-row gap-4 w-full h-10 bg-white justify-center items-center rounded-md text-sm">
            <div className="flex flex-row gap-1 items-center">
              <span>Address</span>
              <span className="bg-slate-500 rounded text-gray-200 px-2">{address}</span>
              <IoShuffleSharp className="cursor-pointer" onClick={() => shuffleAddresses()} />
            </div>
            <div className="flex flex-row gap-1">
              <span>Port</span>
              <span className="bg-slate-500 rounded text-gray-200 px-2">3310</span>
            </div>
          </div>
        </div>

        <div className="flex-1 flex flex-col p-2 bg-[#131415]">
          <div className="text-white flex flex-row justify-between items-center gap-2 font-bold">
            {/* <div className="text-white flex flex-row justify-between items-center bg-gray-800 rounded-lg p-2 gap-2"> */}
            <span className="bg-gray-500 p-3 px-4 rounded cursor-pointer hover:bg-gray-700 flex-1 text-center no-select ">
              LOGS
            </span>
            <span className="bg-gray-500 p-3 px-4 rounded cursor-pointer hover:bg-gray-700 flex-1 text-center no-select ">
              STATES
            </span>
            <span className="bg-gray-500 p-3 px-4 rounded cursor-pointer hover:bg-gray-700 flex-1 text-center no-select ">
              EVENTS
            </span>
            <span className="bg-gray-500 p-3 px-4 rounded cursor-pointer hover:bg-gray-700 flex-1 text-center no-select ">
              FLOW
            </span>
          </div>
          {events.length > 0 ? (
            <ul>
              {events.map((event: any, index: any) => (
                <li key={index}>{event}</li>
              ))}
            </ul>
          ) : null}

          {/* <ReactDataViewer data={[1, 2, "something@other.com", { fasf: "sdfasf", lofdasf: true }]} /> */}
        </div>
      </div>
    </div>
  );
}

export default App;
