/// <reference types="vite/client" />

declare module "*.css";
declare module "*.png";
declare module "*.svg";
declare module "*.jpg";

interface ImportMetaEnv {
    readonly VITE_GOOGLE_CALENDAR_API_KEY: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
