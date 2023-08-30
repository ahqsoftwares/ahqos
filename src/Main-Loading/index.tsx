import "./index.css";

interface LoadingScreen {
    value?: number;
}

export default function KernelLoading(props: LoadingScreen) {
    return (
        <div className="kernel-main">
            <img src="/icon.png" />

            <progress className="progress w-72" value={props.value} max="100"></progress>
        </div>
    )
}