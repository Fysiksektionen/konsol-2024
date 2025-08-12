import { Dispatch, SetStateAction } from "react";

interface SlideData {
    id: string;
    caption: string;
    start_date: Date;
    end_date: Date;
    active: boolean;
    filetype: string;
}

interface UserData {
    id: string;
    email: string;
    admin: boolean;
}

interface User {
    email: string;
    permission: 'Admin' | 'User';
}

export type { SlideData, UserData, User };
