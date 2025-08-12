import React from 'react';
import '../styles/Header.css'
import { User } from '../types';
import UserStatus from './UserStatus';


type Props = {
  user: User | null | undefined;
  setUser: (user: User | null | undefined) => void;
}

const Header = (props: Props) => {
  const { user, setUser } = props;
  return (
    <div className="header">
      <div className="header-left">
        <img src="https://f.kth.se/wp-content/uploads/FysikMedium.png" className="logo" />
        <h1 className='title'>Konsol Admin</h1>
      </div>
      <div className="header-right">
        <div className="user-status">
          <UserStatus user={user} setUser={setUser} />
        </div>
      </div>

    </div>
  )
}

export default Header