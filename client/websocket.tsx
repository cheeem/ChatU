import { x500, websocket, messages, userIndex, mainState, } from "./app";
import { UserContacts } from "./contacts";

type ClientEvent = keyof typeof CLIENT_EVENT_MAP;
type ServerEvent = ServerMessage | ServerJoin | ServerLeave | ServerConnectRequest | ServerConnectSuccess | ServerConnectFailure;

type ServerMessage = { readonly type: "Message", readonly data: Message, };
type ServerJoin = { readonly type: "Join", readonly data: number, };
type ServerLeave = { readonly type: "Leave", readonly data: number, };
type ServerConnectRequest = { readonly type: "ConnectRequest" };
type ServerConnectSuccess = { readonly type: "ConnectSuccess", readonly data: ReadonlyArray<UserContacts>, };
type ServerConnectFailure = { readonly type: "ConnectFailure" };

type Message = { user_idx: number, content: string, };

const CLIENT_EVENT_MAP = {
	"Skip": 0,
	"Leave": 1,
	"Connect": 2,
} as const;

const USER_COLORS = [
	"blue",
	"purple",
	"green",
	"orange",
] as const;

export function join(/*websocket: WebsocketSignal, messages: MessagesSignal, userIndex: UserIndexSignal, mainState: MainStateSignal, x500: string*/) {

	try {
		websocket.value = new WebSocket(`ws://localhost:3000/join?x500=${x500}`);
		mainState.value = "loading";
	} catch(error) {
		return;
	}

	websocket.value.onclose = () => leave(/*websocket, mainState*/);
	websocket.value.onmessage = (e: MessageEvent) => setUserIndex(e, /*websocket, messages, userIndex, mainState*/);

}

export function setUserIndex(e: MessageEvent, /*websocket: WebsocketSignal, messages: MessagesSignal, userIndex: UserIndexSignal, mainState: MainStateSignal*/) {
	
    userIndex.value = parseInt(e.data);
    
    messages.value = [];
	
    websocket.value!.onmessage = (e: MessageEvent) => messageReceived(e, /*messages, userIndex*/);
	
    mainState.value = "chat";

}

export function sendEvent(/*websocket: WebsocketSignal,*/ event: ClientEvent) {
	
	if(!websocket.value) {
		return;
	}

	const uint8: Uint8Array = new Uint8Array(1);
	uint8[0] = CLIENT_EVENT_MAP[event];
	websocket.value.send(uint8.buffer);

}

export function message(/*websocket: WebsocketSignal,*/ input: HTMLInputElement) {
	
	if(!websocket.value) {
		return;
	}

	websocket.value.send(input.value);
	
	input.value = "";

}

function messageReceived(e: MessageEvent, /*messages: MessagesSignal, userIndex: UserIndexSignal*/) {

    const serverEvent: ServerEvent = JSON.parse(e.data) as ServerEvent;

    console.log(serverEvent);

    if(serverEvent.type === "Message"){
        messages.value = [...messages.value, (
			<li 
				class={userIndex.value === serverEvent.data.user_idx ? "right" : "left"}
				style={`--user-color: ${USER_COLORS[serverEvent.data.user_idx]}`}
			> 
				{serverEvent.data.content} 
			</li>
		)];
    }

}

function leave(/*websocket: WebsocketSignal, mainState: MainStateSignal*/) {
	
	websocket.value = null;

	mainState.value = "join";
	
}
