import "./chat.css";

import { createRef } from "preact";

import { websocket, messages, userIndex } from "../app";
import { message, sendEvent, setUserIndex, ChatEvent } from "../websocket";

const USER_COLORS = [
	"blue",
	"red",
	"green",
	"orange",
] as const;

export default function Chat() {
    
    const text = createRef<HTMLInputElement>();

    return (
        <div id="chat">

            <div id="actions"> 

            <input placeholder="type something..." 
                ref={text}
                onKeyDown={(e) => {
                    if(e.key !== "Enter") return;
                    message(text.current!);
                }}
            />

            <button type="button"
                onClick={() => message(text.current!)}
            > Send </button>

            <button type="button"
                onClick={() => {
                    websocket.value!.onmessage = setUserIndex;
                    sendEvent("Skip")
                }}
            > Skip </button>

            <button type="button"
                onClick={() => sendEvent("Leave")}
            > Leave </button>

            </div>
	
				<ul id="display"> 
                    {messages.value.map(toChatComponent)}
                </ul>
	
		</div>
    );

}

function toChatComponent(event: ChatEvent): preact.JSX.Element {

    if(event.type === "Message") {
        return (
            <li 
                class={userIndex.value === event.data.user_idx ? "message right" : "message left"}
                style={`--user-color: ${USER_COLORS[event.data.user_idx]}`}
            > 
                {event.data.content} 
            </li>
        );
    }
    
    if(event.type === "Join") {
        return (
            <li 
                class={userIndex.value === event.data ? "join right" : "join left"}
                style={`--user-color: ${USER_COLORS[event.data]}`}
            > 
                {userIndex.value === event.data ? "You" : "They"} Joined
            </li>
        );
    }

    if(event.type === "Leave") {
        return (
            <li 
                class={userIndex.value === event.data ? "leave right" : "leave left"}
                style={`--user-color: ${USER_COLORS[event.data]}`}
            > 
                {userIndex.value === event.data ? "You" : "They"} Left
            </li>
        )
    }

    return <></>

}