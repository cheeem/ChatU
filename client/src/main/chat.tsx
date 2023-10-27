import "./chat.css";

import { createRef } from "preact";

import { websocket, messages } from "../app";
import { message, sendEvent, setUserIndex } from "../websocket";

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
                    {messages.value}
                </ul>
	
		</div>
    );

}