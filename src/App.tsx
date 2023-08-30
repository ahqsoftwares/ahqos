import { useEffect, useState } from "react";
import { Page, kernelInit, Users } from "./utils";

// Tauri Apis
import { exit } from "@tauri-apps/api/process";
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/api/notification";

// Core Pages
import KernelLoading from "./UI/Main-Loading";

async function bootError() {
    let permission = await isPermissionGranted();

    if (!permission) {
        const perms = await requestPermission();

        permission = perms === "granted";
    }

    if (permission) {
        sendNotification({
            title: "Underprivileged",
            body: "Unable to boot ahqOS due to insufficient permissions, re-run as \"super-user\" (i.e. \"sudo\")"
        });
    }

    setTimeout(() => {
        exit(1);
    }, 1000);
}

export default function App() {
    const [page, setPage] = useState<Page>("kernel-startup");
    const [progress, setProgress] = useState<number|undefined>();

    const [app, setApp] = useState(<KernelLoading value={0} />);

    const [users, setUsersArray] = useState<Users>([]);

    useEffect(() => {
        kernelInit((data) => setProgress(data))
            .then((data) => {
                if (data) {
                    bootError();
                    setUsersArray(data);
                    setPage("login");
                } else {
                    bootError();
                }
            });
    }, []);

    useEffect(() => {
        if (progress == -1) {
            bootError();
        }

        switch (page) {
            case "kernel-startup":
                document.querySelector("html")?.setAttribute("data-theme", "night");
                document.querySelector("html")?.setAttribute("class", "dark");
                setApp(<KernelLoading value={progress} />);
                break;
            default:
                setApp(<KernelLoading value={-1} />);
        }
    }, [page, progress, users]);

    return app;
}