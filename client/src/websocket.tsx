import { x500, contacts, websocket, messages, userIndex, mainState, } from "./app";
import { Contacts, UserContacts } from "./contacts";

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
	"red",
	"green",
	"orange",
] as const;

export function join() {

	const url: URL = new URL("ws://localhost:8080/join");
	
	url.searchParams.append("x500", x500);

	for(const _field in contacts.value) {

		const field = _field as keyof Contacts;

		if(contacts.value[field]) {
			url.searchParams.append(field, contacts.value[field]!);
		}

	}

	console.log(url)

	try {
		websocket.value = new WebSocket(url);
		mainState.value = "loading";
	} catch(error) {
		return;
	}

	websocket.value.onclose = leave;
	websocket.value.onmessage = setUserIndex;

}

export function setUserIndex(receivedEvent: MessageEvent) {
	
    userIndex.value = parseInt(receivedEvent.data);
    
    messages.value = [];
	
    websocket.value!.onmessage = messageReceived;
	
    mainState.value = "chat";

}

export function sendEvent(event: ClientEvent) {
	
	if(!websocket.value) {
		return;
	}

	const uint8: Uint8Array = new Uint8Array(1);
	uint8[0] = CLIENT_EVENT_MAP[event];
	websocket.value.send(uint8.buffer);

}

export function message(input: HTMLInputElement) {
	
	if(!input.value) {
		return;
	}

	if(!websocket.value) {
		return;
	}

	websocket.value.send(input.value);
	
	input.value = "";

}

function messageReceived(receivedEvent: MessageEvent) {

    const serverEvent: ServerEvent = JSON.parse(receivedEvent.data) as ServerEvent;

    console.log(serverEvent);

    if(serverEvent.type === "Message"){
        messages.value = [...messages.value, (
			<li 
				class={userIndex.value === serverEvent.data.user_idx ? "message right" : "message left"}
				style={`--user-color: ${USER_COLORS[serverEvent.data.user_idx]}`}
			> 
				{serverEvent.data.content} 
			</li>
		)];
    }

	if(serverEvent.type === "Join"){
        messages.value = [...messages.value, (
			<li 
				class={userIndex.value === serverEvent.data ? "join right" : "join left"}
				style={`--user-color: ${USER_COLORS[serverEvent.data]}`}
			> 
				{userIndex.value === serverEvent.data ? "You" : "They"} Joined
			</li>
		)];
    }

	if(serverEvent.type === "Leave"){
        messages.value = [...messages.value, (
			<li 
				class={userIndex.value === serverEvent.data ? "leave right" : "leave left"}
				style={`--user-color: ${USER_COLORS[serverEvent.data]}`}
			> 
				{userIndex.value === serverEvent.data ? "You" : "They"} Left
			</li>
		)];
    }

}

function leave() {
	
	websocket.value = null;

	mainState.value = "join";
	
}
