import React, { useEffect, useState, useContext } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { relaunch } from "@tauri-apps/api/process";
import { open } from "@tauri-apps/api/shell";
import "../App.css";

import { IoShuffleSharp } from "react-icons/io5";

import { TConfig } from "../types/config";
import { SSE } from "../types/enums";
import { DataViewer, Menu } from "../components/layout";
import { useAppDispatch, useAppSelector } from "../redux/hooks";
import { getPreferences } from "../commands";
import { setPreferences } from "../redux/slices/configSlice";

import { appWindow } from '@tauri-apps/api/window';
import { Link, useNavigate } from "react-router-dom";

import { IoChevronForward } from "react-icons/io5";
import { ThemeToggle } from "@/components/form-elements";


export default function Home() {

  const dispatch = useAppDispatch();
  const navigate = useNavigate()
  const { preferences } = useAppSelector((state: any) => state.config);

  type TPayload = {
    event:any;
    payload:string;
  }

  useEffect(() => {
    (async() => {

      if(!preferences) {
        const prefs = await getPreferences();
        dispatch(setPreferences(prefs));
      }

      await appWindow.listen(
        'PROGRESS',
        ({event, payload}:TPayload) => {
          setEvents((events: any) => [...events, payload]);
        }
      );

      await appWindow.listen(
        'LOADING',
        ({event, payload}:TPayload) => {
          setEvents((events: any) => [...events, payload]);
          console.log("LOADING")
        }
      );

      await appWindow.listen(
        'OPEN',
        ({event, payload}:TPayload) => {
          console.log('OPEN',payload)
          if(payload === "preferences"){
            navigate("/preferences");
          }
        }
      );

    })()
    
  },[])

  const [logs,setLogs] = useState(null);
  const [states,setState] = useState(null);
  const [flow,setFlow] = useState(null);

  const [greetMsg, setGreetMsg] = useState("");
  const [ip, setIP] = useState("");
  const [port, setPort] = useState("");
  const [name, setName] = useState("");

  const [config, setConfig] = useState<TConfig|null>(null);

  const [sseConnection, setSSEConnection] = useState<SSE>(SSE.IDLE);

  const [connections,setConnections] = useState<string[]|unknown[]>([]);
  const [selectedConnection,setSelectedConnection] = useState<string>("");

  const [serverRunning,setServerRunning] = useState<boolean>(false);

  const [loading,setLoading] = useState<boolean>(false);
  // const connectionsStateRef = React.useRef(connections);

  // const keepTrackOfConnections = (con: any)  => {
  //   connectionsStateRef.current = con;
  //   setConnections((prev) => [...prev,con]);
  // }

 
  async function do_some_long_task() {
    await invoke("do_some_long_task");
  }
  async function is_server_running() {
    setServerRunning(await invoke("is_server_running"));
  }

  async function start_my_server() {
    setLoading(true);
    console.log("before")
    await invoke("start_my_server");
    setLoading(false);
    console.log("after")
  }

  async function shout(e: string) {
    setGreetMsg(await invoke("shout", { phrase: e }));
  }

  // const inpRef: any = React.useRef(null);

  const [events, setEvents] = useState<any>([]);
  const [filteredEvents, setFilteredEvents] = useState<any>([]);

  React.useEffect(() => {
    setFilteredEvents(events);
    const cons : string[]|unknown[] = [...new Set(events.map((item:any) => JSON.parse(item).connection))];
    setConnections(cons);
  }, [events]);

  const selectEvent = (con:any) => {
    setSelectedConnection(con);
    setFilteredEvents(events.filter((e:any) => JSON.parse(e).connection === con ));
  }

  React.useEffect(() => {
    if (name) shout(name);
    else setGreetMsg("");

    // inpRef.current.focus();
  }, [name]);

  async function getIP() {
    setIP(await invoke("my_ip"));
  }

  async function getConfig() {
    setConfig(await invoke("cmd_get_config"));
  }

  async function getPort() {
    console.log("getPort")
    setPort(await invoke("my_port"));
  }

  React.useEffect(() => {
    if(!ip) getIP();
    if(!port) getPort();
    getConfig();
  }, []);

  const handleSSE = () => {
    if ("EventSource" in window && config) {
      const eventSource = new EventSource(`http://127.0.0.1:${config.port}/events`);

      eventSource.onopen = () => {
        console.log("Connection Opened");
        setSSEConnection(SSE.CONNECTION_ESTABLISHED);
      };

      eventSource.onmessage = (event) => {
        console.log(JSON.parse(event.data));
        setEvents((events: any) => [...events, event.data]);
        const con = JSON.parse(event.data).connection;
        console.log("connections.includes(con)",!connections.includes(con))
        console.log("connections",connections)
        if(!connections.includes(con)){
          setConnections((prev) => [...prev,con]);
        }
      };

      // eventSource.onmessage = (event) => {
      //   const parsedData = JSON.parse(event.data);

      //   setEvents((events:any) => events.concat(parsedData));
      // };

      eventSource.onerror = (event) => {
        console.error(event);
        eventSource.close();
        setSSEConnection(SSE.CONNECTION_ERROR);
      };

      return () => {
        eventSource.close();
        setSSEConnection(SSE.IDLE);
      };
    } else {
      // throw fatal error
      console.log("Could not connect to SSE")
      setSSEConnection(SSE.CONNECTION_ERROR);
    }
  }

  React.useEffect(() => {
    // if(config) handleSSE()
  }, [config]);

  return (
    <div className="flex-grow flex flex-row overflow-hidden justify-center h-screen overscroll-none">
      <div className="flex-shrink-0 w-64 bg-gray-200 dark:bg-[#1e1f21] flex flex-col border-solid border-r-2 border-[#0e0e0f]">
        <div className="flex flex-col flex-1 p-2 gap-2 scrollbar-thin scrollbar-thumb-[rgba(255,255,255,.05)] scrollbar-track-[#1e1f21] overflow-y-auto">
         
          { connections && connections.map((con:any,key) => {
          return (
            <div key={key} onClick={() => selectEvent(con)} className={`text-gray-400 bg-[#131415] rounded-md p-3 cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] ${con === selectedConnection ? 'bg-gray-400 text-[#1e1f21]':''}`}>
              {con}
            </div>
          )
          })}

        </div>

        <div className="p-2">
          <Link to="/preferences" className="flex h-10 bg-[#131415] cursor-pointer justify-between group hover:bg-gray-400  px-2 items-center rounded-md text-sm no-select">
              <span
                className="text-gray-200 group-hover:text-[#1e1f21]"
                title="Click to Copy"
                
              >
                Settings 
              </span>
              <IoChevronForward className="text-gray-200 group-hover:text-[#1e1f21]"/>
          </Link>
        </div>
      </div>

      <div className="flex-1 flex flex-col bg-gray-50 dark:bg-[#131415]">
        <Menu/>
        <div className="flex flex-col gap-2 flex-1 p-2 scrollbar-thin scrollbar-thumb-[rgba(255,255,255,.1)] scrollbar-track-[#131415] hover:scrollbar-thumb-gray-400 overflow-y-auto">
          {/* <p className="text-white">{JSON.stringify(preferences,null,2)}</p> */}
          <p className="text-white">Server Running: {JSON.stringify(serverRunning)}</p>
          {/* <button onClick={() => do_some_long_task()}>Click here</button> */}
          <button className="dark:bg-gray-50 dark:text-gray-900 bg-[#131415] text-gray-50" onClick={() => start_my_server()}>{loading ? "Loading..." : "Start Server" }</button>
          <ThemeToggle/>
          <DataViewer filteredEvents={filteredEvents}/>
        </div>
      </div>
    </div>
  );
}
