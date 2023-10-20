import "./app.css";

import { createRef, } from "preact";
import { signal, Signal, } from "@preact/signals";
import { useState } from "preact/hooks";
import { SetStateAction } from "preact/compat";

import { nanoid } from "nanoid";

import Join from "./main/join";
import Chat from "./main/join";

const CLIENT_EVENT_MAP = {
	"SKIP": 0,
	"LEAVE": 1,
	"CONNECT": 2,
} as const;

type ClientEvent = keyof typeof CLIENT_EVENT_MAP;
type MainState = "join" | "chat";

const x500: string = nanoid(8);

const websocket: Signal<WebSocket | null> = signal(null);
const messages: Signal<string> = signal("");
const mainState: Signal<MainState> = signal("join");

export default function App() {

	const text = createRef<HTMLInputElement>();

	if(mainState.value === "chat") {

		return (
			<>
	
				<textarea id="messages" cols={30} rows={10}
					value={messages}
				></textarea>
	
				<input placeholder="type something..." 
					ref={text}
					onKeyDown={(e) => e.key === "Enter" && message(websocket, text.current!)} 
				/>
	
				<button type="button"
					onClick={() => message(websocket, text.current!)}
				> Send </button>
	
				<button type="button"
					onClick={() => event(websocket, "SKIP")}
				> Skip </button>
	
				<button type="button"
					onClick={() => event(websocket, "LEAVE")}
				> Leave </button>
	
				<button type="button"
					onClick={() => event(websocket, "CONNECT")}
				> Connect </button>
	
			</>
		);
	
	}

	return <button type="button" 
		onClick={() => join(websocket, mainState, messages, x500)}
	> Join Chat </button>

	// return <Join 
	// 	join={join} 
	// 	websocket={websocket} 
	// 	mainState={mainState}
	// 	messages={messages} 
	// 	x500={x500} 
	// />;

}

function event(websocket: Signal<WebSocket | null>, event: ClientEvent) {
	
	if(!websocket.value) {
		return;
	}

	const uint8: Uint8Array = new Uint8Array(1);
	uint8[0] = CLIENT_EVENT_MAP[event];
	websocket.value.send(uint8.buffer);

}

function leave(websocket: Signal<WebSocket | null>, mainState: Signal<MainState>) {
	
	websocket.value = null;

	mainState.value = "join";
	
}

function messageReceived(e: MessageEvent, messages: Signal<string>) {

	messages.value += e.data+"\r\n";

}

function message(websocket: Signal<WebSocket | null>, input: HTMLInputElement) {
	
	if(!websocket.value) {
		return;
	}

	websocket.value.send(input.value);
	
	input.value = "";

}

function join(websocket: Signal<WebSocket | null>, mainState: Signal<MainState>, messages: Signal<string>, x500: string) {

	console.log("hi")

	websocket.value = new WebSocket(`ws://localhost:3000/join?x500=${x500}`);
	websocket.value.onclose = () => leave(websocket, mainState);
	websocket.value.onmessage = (e: MessageEvent) => messageReceived(e, messages);

	mainState.value = "chat";

	console.log(websocket.value, mainState.value)

}
