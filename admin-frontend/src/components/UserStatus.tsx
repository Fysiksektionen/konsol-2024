import React from 'react'
import { User } from '../types'
import { GoogleLogin } from '@react-oauth/google';

type Props = {
    user: User | null | undefined
    setUser: (user: User | null | undefined) => void;
}

const UserStatus = ({ user, setUser }: Props) => {

    const logout = () => {
        fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/logout`, {
            method: 'POST',
            credentials: 'include',
        }).then(() => {
            setUser(null);
        }).catch(err => {
            console.error('Logout failed:', err);
        });
    };

    if (user === undefined) {
        return <p>Loading...</p>
    } else if (user === null) {
        return <GoogleLogin
            onSuccess={credentialResponse => {
                const body: { id_token: string } = { id_token: credentialResponse.credential as string };
                fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/verify`, {
                    method: 'POST',
                    credentials: 'include',
                    headers: {
                        'Content-Type': 'application/json',
                    },

                    body: JSON.stringify(body),
                }).then(res => {
                    res.json().then(body => {
                        console.log('Login successful');
                        setUser({ email: body.email, permission: body.permission });
                    })
                }).catch(err => {
                    console.error(err);
                });
            }}

            onError={() => {
                console.log('Login Failed');
            }}
            useOneTap />
    } else {
        return (<>
            <p onClick={logout}>{user.email}</p>
            {user.permission === "Admin" ? <p>Admin</p> : <></>}
        </>)
    }
}

export default UserStatus