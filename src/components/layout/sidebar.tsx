import { checkAppUpdate } from '@/commands'
import React from 'react'
import { IoChevronBack, IoChevronForward, IoSyncSharp } from 'react-icons/io5'
import { Link } from 'react-router-dom'
import logo from '@/assets/128x128.png';

interface IProps {
  back?: boolean;
  children?: React.ReactElement | JSX.Element[] | React.ReactNode;
  [x: string]: any; // => all other props (...rest)
}

export default function Sidebar(props: IProps) {

  const {children, back} = props;
  return (
    <div className="flex-shrink-0 w-64 bg-gray-200 dark:bg-[#1e1f21] flex flex-col border-solid border-r-2 border-[#0e0e0f]">
        {/* <div className="flex flex-row justify-between items-center text-gray-400 min-h-14 max-h-14 bg-[#131415] p-3 cursor-pointer pb-2 border-b-2 border-[#0e0e0f]">
          <img className="img-responsive w-8" src={logo} alt="{dev.all.day}"/>
          <span className="text-1xl font-bold text-gray-100">{"{dev.all.day}"}</span>
          <span onClick={checkAppUpdate} className="text-sm text-gray-700 bg-gray-300 rounded-md px-2 flex justify-center items-center gap-2 hover:bg-gray-500 hover:text-gray-50">1.0.1 <IoSyncSharp className="font-bold"/></span>
        </div> */}
        <div className="flex flex-col flex-1 p-2 gap-2 scrollbar-thin scrollbar-thumb-[rgba(255,255,255,.05)] scrollbar-track-[#1e1f21] overflow-y-auto">
         
          {/* { connections && connections.map((con:any,key) => {
          return (
            <div key={key} onClick={() => selectEvent(con)} className={`text-gray-400 bg-[#131415] rounded-md p-3 cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] ${con === selectedConnection ? 'bg-gray-400 text-[#1e1f21]':''}`}>
              {con}
            </div>
          )
          })} */}

          {children}

        </div>

        {/* <div className="p-2">
          <Link to={back ? "/" : "/preferences"} className={`flex h-10 bg-[#131415] cursor-pointer justify-${back ? 'center' : 'between'} group hover:bg-gray-400  px-2 items-center rounded-md text-sm no-select`}>
              {back ? <>
                <IoChevronBack className="text-gray-200 group-hover:text-[#1e1f21]"/>
              </> : 
              <>
              <span
                className="text-gray-200 group-hover:text-[#1e1f21]"
                title="Click to Copy"
                
              >
                Settings 
              </span>
              <IoChevronForward className="text-gray-200 group-hover:text-[#1e1f21]"/>
              </>}
          </Link>
        </div> */}
      </div>
  )
}
