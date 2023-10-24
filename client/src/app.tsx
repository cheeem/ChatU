import "./app.css";

import { signal, Signal, } from "@preact/signals";

import { nanoid } from "nanoid";

import Join from "./main/join";
import Chat from "./main/chat";

export type MainState = "join" | "chat" | "loading";

export type Message = { user_idx: number, content: string, };

export type WebsocketSignal = Signal<WebSocket | null>;
export type MessagesSignal = Signal<Message[]>;
export type UserIndexSignal = Signal<number | null>;
export type MainStateSignal = Signal<MainState>;

const x500: string = nanoid(8);

const websocket: WebsocketSignal = signal(null);
const messages: MessagesSignal = signal([]);
const userIndex: UserIndexSignal = signal(null);
const mainState: MainStateSignal = signal("join");

export default function App() {

	if(mainState.value === "chat") {

		return (
			<Chat
				websocket={websocket}
				messages={messages}
				userIndex={userIndex}
				mainState={mainState}
			/>
		);
	
	}

	return (
		<Join 
			websocket={websocket} 
			messages={messages} 
			userIndex={userIndex} 
			mainState={mainState} 
			x500={x500} 
		/>
	);

}

