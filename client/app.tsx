import "./app.css";

import { signal, Signal, } from "@preact/signals";

import { nanoid } from "nanoid";

import Join from "./main/join";
import Chat from "./main/chat";

export type MainState = "join" | "chat" | "loading";

export const x500: string = nanoid(8);
export const websocket: Signal<WebSocket | null> = signal(null);
export const messages: Signal<preact.JSX.Element[]> = signal([]);
export const userIndex: Signal<number | null> = signal(null);
export const mainState: Signal<MainState> = signal("join");

export default function App() {

	if(mainState.value === "chat") {
		return <Chat />;
	}

	if(mainState.value === "loading") {
		return <p> loading... </p>
	}

	if(mainState.value === "join") {
		return <Join />;
	}

}

