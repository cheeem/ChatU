import "./chat.css";

import { createRef } from "preact";

import { message, sendEvent, setUserIndex } from "../websocket";
import { Message, WebsocketSignal, MessagesSignal, UserIndexSignal, MainStateSignal } from "../app";

type ChatParameters = {
    websocket: WebsocketSignal,
    messages: MessagesSignal,
    userIndex: UserIndexSignal,
    mainState: MainStateSignal,
}

export default function Chat({ websocket, messages, userIndex, mainState, }: ChatParameters) {
    
    const text = createRef<HTMLInputElement>();

    console.log(userIndex.value);

    return (
        <div id="chat">
	
				<ul id="display"> 
                    {messages.value.map((message: Message) => 
                        <li class={userIndex.value === message.user_idx ? "right" : "left"}> 
                            {message.content} 
                        </li>
                    )}
                </ul>

                <div id="actions"> 

                    <input placeholder="type something..." 
                        ref={text}
                        onKeyDown={(e) => {
                            if(e.key !== "Enter") return;
                            message(websocket, text.current!);
                        }}
                    />
        
                    <button type="button"
                        onClick={() => message(websocket, text.current!)}
                    > Send </button>
        
                    <button type="button"
                        onClick={() => {
                            websocket.value!.onmessage = (e: MessageEvent) => setUserIndex(e, websocket, messages, userIndex, mainState);
                            sendEvent(websocket, "Skip")
                        }}
                    > Skip </button>
        
                    <button type="button"
                        onClick={() => sendEvent(websocket, "Leave")}
                    > Leave </button>
	
                    <button type="button"
                        onClick={() => sendEvent(websocket, "Connect")}
                    > Connect </button>
	
                </div>
	
			</div>
    );

}