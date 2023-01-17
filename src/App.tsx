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

  const [logs,setLogs] = useState(null);
  const [states,setState] = useState(null);
  const [flow,setFlow] = useState(null);

  const [greetMsg, setGreetMsg] = useState("");
  const [ip, setIP] = useState("");
  const [name, setName] = useState("");


  const [connections,setConnections] = useState<string[]>([]);
  const [selectedConnection,setSelectionConnection] = useState<string>("");
  const connectionsStateRef = React.useRef(connections);

  const keepTrackOfConnections = (con: any)  => {
    connectionsStateRef.current = con;
    setConnections((prev) => [...prev,con]);
  }

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
  const [filteredEvents, setFilteredEvents] = useState<any>([]);

  React.useEffect(() => {
    setFilteredEvents(events);
  }, [events]);

  const selectEvent = (con:any) => {
    setSelectionConnection(con);
    setFilteredEvents(events.filter((e:any) => JSON.parse(e).connection === selectedConnection ));
  }

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
        const con = JSON.parse(event.data).connection;
        console.log("connections.includes(con)",!connections.includes(con))
        console.log("connections",connections)
        if(!connectionsStateRef.current.includes(con)){
          keepTrackOfConnections(con);
        }
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
    <div className="flex-grow flex flex-row overflow-hidden justify-center h-screen overscroll-none">
      <div className="flex-shrink-0 w-72 bg-[#1e1f21] flex flex-col border-solid border-r-2 border-[#0e0e0f]">
        <div className="flex flex-col flex-1 p-2 gap-2 scrollbar-thin scrollbar-thumb-[rgba(255,255,255,.05)] scrollbar-track-[#1e1f21] overflow-y-auto">
          { connections && connections.map((con,key) => {
          return (
            <div key={key} onClick={() => selectEvent(con)} className={`text-gray-400 bg-[#131415] rounded-md p-3 cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] ${con === selectedConnection ? 'bg-gray-400 text-[#1e1f21]':''}`}>
              {con}
            </div>
          )
          })}
        </div>

        <div className="p-2">
          <div className="flex flex-row gap-4  h-10 bg-[#131415] justify-center items-center rounded-md text-sm no-select cursor-default">
            <div className="flex flex-row gap-1 items-center text-gray-400">
              <span className="text-[10px] uppercase">Host</span>
              <span
                className="bg-[#1e1f21] hover:bg-gray-400 hover:text-[#1e1f21] rounded text-gray-200 px-2 cursor-pointer"
                title="Click to Copy"
              >
                {address}
              </span>
              <IoShuffleSharp className="cursor-pointer text-gray-400" onClick={() => shuffleAddresses()} />
            </div>
            <div className="flex flex-row gap-1 text-gray-400">
              <span className="text-[10px] uppercase">Port</span>
              <span
                className="bg-[#1e1f21] hover:bg-gray-400 hover:text-[#1e1f21] rounded text-gray-200 px-2 cursor-pointer"
                title="Click to Copy"
              >
                3310
              </span>
            </div>
          </div>
        </div>
      </div>

      <div className="flex-1 flex flex-col bg-[#131415]">
        <div className="text-gray-400 flex flex-row justify-between items-center gap-2 my-2 px-2 font-bold pb-2 border-b-2 border-[#0e0e0f]">
          {/* <div className="text-white flex flex-row justify-between items-center bg-gray-800 rounded-lg p-2 gap-2"> */}
          <span className="bg-[#1e1f21] p-3 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
            LOGS
          </span>
          <span className="bg-[#1e1f21] p-3 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
            STATES
          </span>
          <span className="bg-[#1e1f21] p-3 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
            EVENTS
          </span>
          <span className="bg-[#1e1f21] p-3 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
            FLOW
          </span>
        </div>

        <div className="flex flex-col gap-2 flex-1 p-2 scrollbar-thin scrollbar-thumb-[rgba(255,255,255,.1)] scrollbar-track-[#131415] hover:scrollbar-thumb-gray-400 overflow-y-auto">
          {filteredEvents.length > 0
            ? filteredEvents.map((event: any, index: any) => (
                <div className="flex flex-col text-gray-400 text-md bg-[#1e1f21] p-4 rounded-md gap-2" key={index}>
                  <div className="flex justify-between items-center">
                  <span>{JSON.parse(event).time}</span>
                  <span className="bg-gray-400 text-sm text-[#131415] px-2 rounded cursor-pointer no-select hover:bg-[#131415] hover:text-gray-400">Hide</span>
                  </div>
                  <div className="bg-[#131415] p-4 rounded-md">{event}</div>
                </div>
              ))
            : null}
        </div>
      </div>
    </div>
  );
}

export default App;
