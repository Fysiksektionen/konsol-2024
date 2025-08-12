import { UserData, SlideData } from "../types";

export const updateSlides = (setSlides: (slides: SlideData[]) => void) => {
    fetch(`${import.meta.env.VITE_API_BASE_URL}/api/screen/slides`, {
        method: 'GET',
        credentials: 'include'
    })
        .then(response => response.json())
        .then(json => setSlides(json))
        .catch(error => console.error('Error fetching slides:', error));
    // setSlides([{ id: 'dummy-id', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id2', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id3', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }]);
}

export const updateUsers = (setUsers: (users: UserData[]) => void) => {
    fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/list_users`, {
        method: 'GET',
        credentials: 'include'
    })
        .then(response => response.json())
        .then(json => setUsers(json))
        .catch(error => console.error('Error fetching users:', error));
    // setUsers([{ id: 'dummy-id', email: 'user1@example.com', admin: true }, { id: 'dummy-id2', email: 'user2@example.com', admin: false }, { id: 'dummy-id3', email: 'user3@example.com', admin: true }]);
}