import React from 'react'

export default function Menu() {
  return (
    <div className="text-gray-400 flex max-h-14 h-14 flex-row justify-between items-center gap-2 px-2 font-bold  border-b-2 border-[#0e0e0f]">
        <span className="bg-[#1e1f21] p-2 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
        LOGS
        </span>
        <span className="bg-[#1e1f21] p-2 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
        STATES
        </span>
        <span className="bg-[#1e1f21] p-2 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
        EVENTS
        </span>
        <span className="bg-[#1e1f21] p-2 px-4 rounded-md cursor-pointer hover:bg-gray-400 hover:text-[#1e1f21] flex-1 text-center no-select ">
        FLOW
        </span>
    </div>
  )
}
