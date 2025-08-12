import { Routes, Route, Link } from 'react-router-dom'
import Header from './components/Header'
import NavHeader from './components/NavHeader'
import SlidesPage from './pages/SlidesPage'
import './styles/App.css'
import UsersPage from './pages/UsersPage'
import { useEffect, useState } from 'react'
import { User } from './types'
import AccessDeniedPage from './pages/AccessDeniedPage'
import LoadingPage from './pages/LoadingPage'
import NotLoggedInPage from './pages/NotLoggedInPage'

function App() {
  // Undefined if not yet checked, null if checked but not logged in
  const [user, setUser] = useState<User | null | undefined>(undefined);

  useEffect(() => {
    // Check if logged in
    fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/status`, {
      credentials: 'include',
    })
      .then((response) => {
        if (response.ok) {
          return response.json();
        } else {
          throw new Error('Not authenticated');
        }
      })
      .then((user) => {
        setUser(user);
      })
      .catch(() => {
        setUser(null); // Not logged in
      });
  }, []);

  return (
    <div className="app-container">
      <Header user={user} setUser={setUser} />
      <NavHeader user={user} />

      {
      // Session check (see useEffect) in progress
      user === undefined ? <LoadingPage /> :
      // User check finished, not logged in
      user === null ? <NotLoggedInPage /> :
      // User check finished, logged in
      <Routes>
        <Route path="/" element={<SlidesPage />} />
        <Route path="/slides" element={<SlidesPage />} />
        <Route path="/users" element={user?.permission === 'Admin' ? <UsersPage /> : <AccessDeniedPage />} />
      </Routes>
      }
    </div>
  )
}

export default App
