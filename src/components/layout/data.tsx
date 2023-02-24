import React from 'react'
import { IoBanSharp, IoBowlingBallOutline, IoCheckmarkCircleOutline, IoCloseCircleOutline, IoEllipseOutline, IoInformationCircleOutline, IoPlanetOutline } from 'react-icons/io5'
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
    case "warning":
      return "[#856404]"
    case "event":
      return "[#004085]"
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
    case "warning":
      return <IoBanSharp className={`text-${getColor(type)} font-bold text-lg`}/>
    case "event":
      return <IoPlanetOutline className={`text-${getColor(type)} font-bold text-lg`}/>
    default:
      return <IoEllipseOutline className={`text-gray-300 font-bold text-lg`}/>
  }
}

export default function DataViewer({filteredEvents}:any) {
  return (
    <ErrorFallback>
    {filteredEvents.length > 0
      ? filteredEvents.map((event: any, index: any) => (
          <div className="w-full" key={index}>
            <div className="bg-[#191920] flex h-12 justify-between items-center px-4 rounded-t-md">
              <span className="flex-1 text-[#70707d] text-sm font-bold">{JSON.parse(event).description}</span>
              <div className='flex gap-2'>
                <div className="flex gap-2 items-center flex-1">
                  <Icon type={JSON.parse(event).type}/>
                  <span className={`text-${getColor(JSON.parse(event).type)} text-sm uppercase`}>{JSON.parse(event).type}</span>
                </div>
                <span className="flex flex-1 justify-end text-[#70707d] text-sm" >{JSON.parse(event).time}</span>
              </div>
            </div>
            <div className="bg-[#28282f] p-4 text-gray-400 rounded-b-md text-sm">{JSON.stringify(JSON.parse(event),null,4)}</div>
          </div>
        ))
    : null}
  </ErrorFallback>
  )
}
