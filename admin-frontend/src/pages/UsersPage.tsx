import { useEffect, useState } from 'react';
import Popup from 'reactjs-popup';
import 'reactjs-popup/dist/index.css';
import User from '../components/User';
import { UserData } from '../types';
import '../styles/UsersPage.css';
import { updateUsers } from '../util/utils';

type Props = {}

const UsersPage = (props: Props) => {
    const [users, setUsers] = useState<UserData[]>([]);

    useEffect(() => {
      updateUsers(setUsers);
    }, []);

    const handleAddUser = (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault();
      const data = new FormData(event.currentTarget);
      console.log(data);
      const email = data.get('email') as string;
      const admin = data.get('admin') === 'on';

      if (users.some(user => user.email === email)) {
        alert('User already exists');
        return;
      }

      const newUser = { email, permission: admin ? 'Admin' : 'User' };

      fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/add_user`, {
          method: 'POST',
          credentials: 'include',
          headers: {
              'Content-Type': 'application/json',
          },
          body: JSON.stringify(newUser),
      }).then(response => {
        if (!response.ok) {
            throw new Error('Failed to add user');
          }
          return response.json();
        }).then(addedUser => {
          console.log('Added user:', addedUser);
          updateUsers(setUsers); // Refresh users
        });
      }


  return (
    <div className='users-page'>

      <div className="users-header">
        <h1>Users</h1>
        <Popup className="add-user-popup" trigger={<button className='add-user-button'>Add User</button>} modal>
            <h2>Add User</h2>
            <form onSubmit={(event) => handleAddUser(event)}>
              <label>Email:</label>
              <input type="email" name="email" required />
              <label>Admin:</label>
              <input type="checkbox" name="admin" />
              <button type="submit">Submit</button>
            </form>
        </Popup>

      </div>
      {users.map(user => <User userData = {user} setUsers={setUsers} key={user.id} />)}  
    </div>
  )
}

export default UsersPage