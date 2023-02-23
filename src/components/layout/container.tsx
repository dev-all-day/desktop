import React from 'react'

interface IProps {
    children?: React.ReactElement | JSX.Element[] | React.ReactNode;
}

export default function Container({children}: IProps) {
  return (
    <div className="flex-grow flex flex-row overflow-hidden justify-center h-screen overscroll-none">
      {/* Sidebar */}
        {children}
      {/* Body */}
    </div>
  )
}
