import React from 'react'
import Popup from 'reactjs-popup';
import 'reactjs-popup/dist/index.css';
import { UserData } from '../types'
import '../styles/User.css'
import { updateUsers } from '../util/utils';

type Props = {
    userData: UserData
    setUsers: (users: UserData[]) => void;
}

const User = ({ userData, setUsers }: Props) => {
    const handleRemove = () => {
        if (!window.confirm(`Are you sure you want to remove user ${userData.email}?`)) {
            return;
        }

        fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/remove_user`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ id: userData.id }),
        }).then(response => {
            if (response.ok) {
                console.log('User removed successfully');
                updateUsers(setUsers); // Refresh users
            } else {
                console.log('Failed to remove user');
            }
        }).catch(error => {
            console.error('Error removing user:', error);
        });
    }

    return (
        <div className='user'>
            <p>{userData.email}</p>
            <p>{userData.admin ? 'Admin' : 'Not admin'}</p>
            <button className="remove-user-button" onClick={handleRemove}>Remove User</button>

        </div>
  )
}

export default User