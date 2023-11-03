import "./app.css";

import { signal, Signal, } from "@preact/signals";

import { nanoid } from "nanoid";

//main
import Join from "./main/join";
import Chat from "./main/chat";

//sidebar
import Connect from "./sidebar/connect";

import { UserContacts, Contacts } from "./contacts";
import { ChatEvent } from "./websocket";

export type MainState = "join" | "chat" | "loading";
export type ConnectState = "new" | "sent" | "request" | "success" | "failure";

export const x500: string = nanoid(8);
export const contacts: Signal<Contacts> = signal({})
export const websocket: Signal<WebSocket | null> = signal(null);
export const messages: Signal<ChatEvent[]> = signal([]);
export const userIndex: Signal<number | null> = signal(null);
export const mainState: Signal<MainState> = signal("join");
export const connectState: Signal<ConnectState> = signal("new");
export const connectContacts: Signal<ReadonlyArray<UserContacts> | null> = signal(null);

export default function App() {

	if(mainState.value === "chat") {
		return <> <Chat /> <Connect /> </>;
	}

	if(mainState.value === "loading") {
		return <> <div id="loading"> <p> loading... </p> </div> </>
	}

	if(mainState.value === "join") {
		return <> <Join /> </>;
	}

	return <> 404 </>;

}

