import React from 'react'
import { IoBowlingBallOutline, IoCheckmarkCircleOutline, IoCloseCircleOutline, IoEllipseOutline, IoInformationCircleOutline } from 'react-icons/io5'
import { ErrorFallback } from '../app'


const getColor = (type:string) => {
  switch (type) {
    case "error":
      return "[#CC2324]"
    case "success":
      return "[#155724]"
      // return "[#1E7532]"
    case "info":
      return "gray-300"
    case "state":
      return "[#fff3cd]"
    default:
      return "gray-300";
  }
}
const Icon = (props: any) => {
  const { type } = props
  switch (type) {
    case "error":
      return <IoCloseCircleOutline className={`text-${getColor(type)} font-bold text-lg`}/>
    case "success":
      return <IoCheckmarkCircleOutline className={`text-${getColor(type)} font-bold text-lg`}/>
    case "info":
      return <IoInformationCircleOutline className={`text-${getColor(type)} font-bold text-lg`}/>
    case "state":
      return <IoBowlingBallOutline className={`text-${getColor(type)} font-bold text-lg`}/>
    default:
      return <IoEllipseOutline className={`text-gray-300 font-bold text-lg`}/>
  }
}

export default function DataViewer({filteredEvents}:any) {
  return (
    <ErrorFallback>
    {filteredEvents.length > 0
      ? filteredEvents.map((event: any, index: any) => (
          // <div className="flex flex-col text-gray-400 text-md bg-[#1e1f21] p-4 rounded-md gap-2" key={index}>
          //   <div className="flex justify-between items-center">
          //   <span>{JSON.parse(event).time}</span>
          //   <span className="bg-gray-400 text-sm text-[#131415] px-2 rounded cursor-pointer no-select hover:bg-[#131415] hover:text-gray-400">Hide</span>
          //   </div>
          //   <div className="bg-[#28282f] p-4 rounded-md">{event}</div>
          // </div>
          <div className="w-full" key={index}>
            <div className="bg-[#191920] flex h-12 justify-between items-center px-4 rounded-t-md">
              <div className="flex gap-2 items-center flex-1">
                {/* <IoCloseCircleOutline className={`text-${getColor(JSON.parse(event).type)} font-bold text-lg`}/> */}
                <Icon type={JSON.parse(event).type}/>
                <span className={`text-${getColor(JSON.parse(event).type)} text-sm uppercase`}>{JSON.parse(event).type}</span>
              </div>
              <span className="flex-1 text-center text-[#70707d] text-sm">{JSON.parse(event).description}</span>
              <span className="flex flex-1 justify-end text-[#70707d] text-sm" >{JSON.parse(event).time}</span>
            </div>
            <div className="bg-[#28282f] p-4 text-gray-400 rounded-b-md text-sm">{event}</div>
          </div>
        ))
    : null}
  </ErrorFallback>
  )
}
