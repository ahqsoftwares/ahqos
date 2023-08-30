import { invoke } from "@tauri-apps/api/tauri";

type Page = "home" | "login" | "kernel-startup";

enum Theme {
    Light = "Light",
    Dark = "Dark"
}

interface User {
    name: string,
    role: "Administrator" | "General" | "Restricted" | "Guest",
    uid: number,
    passwd: number[],
    theme: {
        apps_theme: Theme,
        tskbr_theme: Theme
    }
}

type Users = User[];

export const delay = (ms: number) => new Promise<void>((resolve) => setTimeout(() => resolve(), ms));

export async function kernelInit(toCall: (arg0: number | undefined) => void) {
    await delay(Math.random() * 8 * 1000);

    const success = await invoke("init_dirs").then(() => true).catch(() => false);

    console.log(success);

    if (!success) {
        toCall(-1);
        return;
    }

    toCall(25);

    await delay(Math.random() * 5 * 1000);

    const config = await invoke<Users>("get_user_config").catch(() => false);

    if (!Array.isArray(config)) {
        toCall(-1);
        return;
    }

    toCall(75);

    await delay(Math.random() * 2.5 * 1000);

    toCall(100);

    return config;
}

export type { Page, Theme, User, Users }