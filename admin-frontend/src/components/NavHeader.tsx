import React from 'react'
import '../styles/NavHeader.css'
import { User } from '../types';

type Props = {
  user: User | null | undefined
}

const NavHeader = ({ user }: Props) => {
  return (
    <div className="nav-header">
        <a href="/slides" className="slides-page-button">Slides</a>
        {user?.permission === 'Admin' && <a href="/users" className="users-page-button">Users</a>}
    </div>
  )
}

export default NavHeader