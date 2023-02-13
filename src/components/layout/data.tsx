import React from 'react'
import { ErrorFallback } from '../app'

export default function DataViewer({filteredEvents}:any) {
  return (
    <ErrorFallback>
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
  </ErrorFallback>
  )
}
