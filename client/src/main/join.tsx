import { join } from "../websocket";
import { WebsocketSignal, MessagesSignal, UserIndexSignal, MainStateSignal } from "../app";

type JoinParameters = {
    websocket: WebsocketSignal,
    messages: MessagesSignal,
    userIndex: UserIndexSignal,
    mainState: MainStateSignal,
    x500: string,
}

export default function Join({ websocket, messages, userIndex, mainState, x500 }: JoinParameters) {
    return (
        <div id="join">
			<button type="button" 
				onClick={() => join(websocket, messages, userIndex, mainState, x500)}
			> Join Chat 
			</button>
			<p> {mainState.value === "loading" ? "loading" : ""} </p>
		</div>
    );

}