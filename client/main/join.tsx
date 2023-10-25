import "./join.css";

import { join } from "../websocket";

export default function Join() {
    return (
        <div id="join">
			<button type="button" 
				onClick={() => join()}
			> Join Chat 
			</button>
		</div>
    );

}