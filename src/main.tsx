// React
import ReactClient from "react-dom/client";

import { appWindow } from "@tauri-apps/api/window";

// Main App
import App from "./App";

// Css
import "./master.css";

appWindow.show();

const root = ReactClient.createRoot(document.getElementById("root") as HTMLDivElement);

root.render(<App />);