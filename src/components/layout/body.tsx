import React from 'react'

interface IProps {
    children?: React.ReactElement | JSX.Element[] | React.ReactNode;
}

export default function Body({children}: IProps) {

  return (
    <div className="flex-1 flex flex-col bg-gray-50 dark:bg-[#0c0c0f]">
        {children}
    </div>
  )
}
