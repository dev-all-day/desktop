import React from "react";
import { ErrorBoundary } from "react-error-boundary";

const FallbackComponent = ({ error, resetErrorBoundary }: any) => {
  return (
    <div className="flex flex-col bg-gray-100 h-screen w-full justify-around items-center">
      <div className="tracking-widest flex flex-col justify-center items-center">
        <span className="text-gray-500 text-6xl block">Something Went Wrong</span>
        <span className="text-gray-500 text-xl my-2">{error.message}</span>
      </div>
      <div className="mt-6 flex justify-center items-center w-full">
        <button
          onClick={resetErrorBoundary}
          className="text-gray-500 font-mono text-xl bg-gray-200 p-3 rounded-md hover:shadow-md"
        >
          Try again
        </button>
      </div>
    </div>
  );
}

export default function ErrorFallback({children} : any) {
    return (<>
    <ErrorBoundary
      FallbackComponent={FallbackComponent}
      onReset={() => {
        // reset the state of your app so the error doesn't happen again
        alert("Tried Again!")
      }} 
    >
      {children}
    </ErrorBoundary>
    </>)
  }
