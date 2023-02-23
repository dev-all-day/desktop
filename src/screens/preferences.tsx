import { ThemeToggle } from '@/components/form-elements'
import { Body, Container, Sidebar } from '@/components/layout'
import React, { useEffect } from 'react'
import { Link } from 'react-router-dom'

export default function PreferencesScreen() {
  
  return (
    <Container>
      <Sidebar back>
      {/* <Link  className='text-gray-50' to="/">Home</Link> */}

      </Sidebar>
      <Body>
      <div>preferences</div>
      <ThemeToggle/>


      </Body>
    </Container>

  )
}
