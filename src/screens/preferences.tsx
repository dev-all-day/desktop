import React, { useEffect } from 'react'
import { Link } from 'react-router-dom'
import { invoke } from "@tauri-apps/api/tauri";

export default function PreferencesScreen() {

  useEffect(() => {
    (async() => {
      await invoke("change_window_title");
    })()
  }, [])
  
  return (
    <div>
      <div>preferences</div>
      <Link  className='text-gray-50' to="/">Home</Link>
    </div>

  )
}
