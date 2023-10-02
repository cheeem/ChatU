import './app.css';
import { createRef, RefObject, } from 'preact';
import { signal, effect, Signal } from "@preact/signals";

export function App() {

	const websocket: Signal<WebSocket | null> = signal(null);
	const joined: Signal<boolean> = signal(false);
	const messages: Signal<string> = signal("");
	const userid = createRef<HTMLInputElement>();

	effect(() => {

		if(!websocket.value) {
			return;
		}

		websocket.value.onclose = () => exit(joined);
		websocket.value.onmessage = (e: MessageEvent) => messageReceived(e, messages);

	});

	return (
		<>

			<input id="userid" type="text" placeholder="userid" 
				disabled={joined}
				ref={userid}
			/>

			<button id="join" type="button" 
				disabled={joined}
				onClick={() => join(websocket, userid, joined)}
			> Join Chat </button>

			<textarea id="messages" cols={30} rows={10}
				value={messages}
			></textarea>

			<input type="text" placeholder="type something..." 
				disabled={!joined} 
				onKeyDown={(e) => handleInput(e, websocket)} 
			/>

			<button id="skip" type="button"
				onClick={() => skip(websocket)}
			> Skip </button>

		</>
	);

}

function exit(disabled: Signal<boolean>) {
	
	console.log("exited");
	disabled.value = false;
	
}

function messageReceived(e: MessageEvent, messages: Signal<string>) {

	console.log("received message: "+e.data);
	messages.value += e.data+"\r\n";

}

function handleInput(e: KeyboardEvent, websocket: Signal<WebSocket | null>) {
	
	if (!websocket.value) {
		return;
	}

	if (e.key === "Enter") {
		const input: HTMLInputElement = e.target as HTMLInputElement;
		websocket.value.send(input.value);
		input.value = "";
	}

}

function skip(websocket: Signal<WebSocket | null>) {
	
	if(!websocket.value) {
		return;
	} 

	websocket.value.send("__skip");

}

function join(websocket: Signal<WebSocket | null>, userid: RefObject<HTMLInputElement>, joined: Signal<boolean>) {
	
	if(userid.current === null) {
		return;
	}

	websocket.value = new WebSocket(`ws://localhost:3000/join?id=${userid.current.value}`);
	console.log("joined");
	joined.value = true;

}
