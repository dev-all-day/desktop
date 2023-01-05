import React, { useEffect, useState } from "react";
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
    <div className="h-screen flex flex-col bg-gray-300">
      <div className="bg-blue-700 p-4">Nav</div>
      <div className="flex-grow flex flex-row overflow-hidden justify-center">
        <div className="flex-shrink-0 w-1/4 p-4">Left menu</div>
        <div className="flex-1 flex flex-col bg-white">
          <div className="border-b-2 m-4 pb-2 border-gray-200">Search</div>
          <main className="flex-1 overflow-y-auto p-4 bg-indigo-200">
            <div className="relative">
              <div className="mb-64">Overflowing content</div>
              <div className="mb-64">Overflowing content</div>
              <div className="mb-64">Overflowing content</div>
              <div className="mb-64">Overflowing content</div>
              <div className="mb-64">Overflowing content</div>
              <div className="mb-64">Overflowing content</div>
            </div>
          </main>
          <div className="flex-shrink-0 w-1/4 p-4">Right sidebar</div>
        </div>
        <div className="bg-blue-700 p-4">Footer</div>
      </div>
    </div>
  );
}

export default App;
