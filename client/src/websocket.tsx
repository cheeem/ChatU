import { x500, contacts, websocket, messages, userIndex, mainState, connectState, connectContacts, } from "./app";
import { Contacts, UserContacts } from "./contacts";

type ClientEvent = keyof typeof CLIENT_EVENT_MAP;
type ServerEvent = ServerMessage | ServerJoin | ServerLeave | ServerConnectRequest | ServerConnectSuccess | ServerConnectFailure;
export type ChatEvent = ServerMessage | ServerJoin | ServerLeave;

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
	"ConnectCancel": 3,
} as const;

export function join() {

	const url: URL = new URL("ws://localhost:8080/join");
	
	url.searchParams.append("x500", x500);

	for(const _field in contacts.value) {

		const field = _field as keyof Contacts;

		if(contacts.value[field]) {
			url.searchParams.append(field, contacts.value[field]!);
		}

	}

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

export function sendEvent(event: ClientEvent, payload?: Uint8Array) {
	
	if(!websocket.value) {
		return;
	}

	let len: number = payload ? payload.length : 0;

	//console.log(payload?.length, len);

	const uint8: Uint8Array = new Uint8Array(1 + len);

	uint8[0] = CLIENT_EVENT_MAP[event];

	for(let i = 0; i < len; i++) {
		//console.log(payload![i])
		uint8[i+1] = payload![i];
	}

	//console.log(uint8);

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

    //console.log(serverEvent);

    if(serverEvent.type === "Message") {
        return messages.value = [...messages.value, serverEvent];
    }

	if(serverEvent.type === "Join") {
        return messages.value = [...messages.value, serverEvent];
    }

	if(serverEvent.type === "Leave") {
        return messages.value = [...messages.value, serverEvent];
    }

	if(serverEvent.type === "ConnectRequest") {
		
		if(connectState.value !== "sent") {
			connectState.value = "request"
		}

		return;

	}

	if(serverEvent.type === "ConnectFailure") {
		return connectState.value = "failure";
	}

	if(serverEvent.type === "ConnectSuccess") {
		//connectContacts.value = null;
		connectContacts.value = serverEvent.data;
		return connectState.value = "success";
	}

}

function leave() {
	
	websocket.value = null;

	mainState.value = "join";
	
}
