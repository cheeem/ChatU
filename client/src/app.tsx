import "./app.css";
import { createRef, } from "preact";
import { signal, effect, Signal, } from "@preact/signals";

const CLIENT_EVENT_MAP = {
	"SKIP": 0,
	"LEAVE": 1,
	"CONNECT": 2,
} as const;

type ClientEvent = keyof typeof CLIENT_EVENT_MAP;

export function App() {

	const websocket: Signal<WebSocket | null> = signal(null);
	const joined: Signal<boolean> = signal(false);
	const messages: Signal<string> = signal("");

	const userid = createRef<HTMLInputElement>();
	const text = createRef<HTMLInputElement>();

	effect(() => {

		if(!websocket.value) {
			return;
		}

		websocket.value.onclose = () => leave(joined);
		websocket.value.onmessage = (e: MessageEvent) => messageReceived(e, messages);

	});

	return (
		<>

			<input type="text" placeholder="userid"
				ref={userid} 
				disabled={joined}
			/>

			<button type="button" 
				disabled={joined}
				onClick={() => join(websocket, joined, userid.current!.value)}
			> Join Chat </button>

			<textarea id="messages" cols={30} rows={10}
				value={messages}
			></textarea>

			<input placeholder="type something..." 
				ref={text}
				disabled={!joined} 
				onKeyDown={(e) => e.key !== "Enter" && message(websocket, text.current!)} 
			/>

			<button type="button"
				disabled={!joined} 
				onClick={() => message(websocket, text.current!)}
			> Send </button>

			<button type="button"
				disabled={!joined} 
				onClick={() => event(websocket, "SKIP")}
			> Skip </button>

			<button type="button"
				disabled={!joined} 
				onClick={() => event(websocket, "LEAVE")}
			> Leave </button>

			<button type="button"
				disabled={!joined} 
				onClick={() => event(websocket, "CONNECT")}
			> Connect </button>

		</>
	);

}

function event(websocket: Signal<WebSocket | null>, event: ClientEvent) {
	
	if(!websocket.value) {
		return;
	}

	const uint8: Uint8Array = new Uint8Array(1);
	uint8[0] = CLIENT_EVENT_MAP[event];
	websocket.value.send(uint8.buffer);

}

function leave(joined: Signal<boolean>) {
	
	joined.value = false;
	
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

function join(websocket: Signal<WebSocket | null>, joined: Signal<boolean>, userid: string) {

	if(joined.value) {
		return;
	}

	websocket.value = new WebSocket(`ws://localhost:3000/join?id=${userid}`);
	joined.value = true;

}
