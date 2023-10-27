import "./join.css";

import { MutableRef, useRef } from "preact/hooks";

import { contacts } from "../app";
import { join } from "../websocket";

type ContactsRef = {
	first_name: MutableRef<null | HTMLInputElement>,
	last_name: MutableRef<null | HTMLInputElement>,
	phone_number: MutableRef<null | HTMLInputElement>,
	instagram: MutableRef<null | HTMLInputElement>,
	snapchat: MutableRef<null | HTMLInputElement>,
	discord: MutableRef<null | HTMLInputElement>,
}

export default function Join() {

	const first_name = useRef(null);
	const last_name = useRef(null);
	const phone_number = useRef(null);
	const instagram = useRef(null);
	const snapchat = useRef(null);
	const discord = useRef(null);

    return (
        <div id="join">

			<form onSubmit={() => {
				setContacts({ first_name, last_name, phone_number, instagram, snapchat, discord, });
				join();
			}}>
				<h3> Provide some contacts below, they're optional </h3>
				<ul> 
					<li> 
						<input type="text" name="first_name" placeholder=" " ref={first_name} />
						<label> First Name </label>
					</li>
					<li> 
						<input type="text" name="last_name" placeholder=" " ref={last_name} />
						<label> Last Name </label>
					</li>
					<li> 
						<input type="text" name="phone_number" placeholder=" " ref={phone_number} />
						<label> Phone Number </label>
					</li>
					<li> 
						<input type="text" name="instagram" placeholder=" " ref={instagram} />
						<label> Instagram </label>
					</li>
					<li> 
						<input type="text" name="snapchat" placeholder=" " ref={snapchat} />
						<label> Snapchat </label>
					</li>
					<li> 
						<input type="text" name="discord" placeholder=" " ref={discord} />
						<label> Discord </label>
					</li>
				</ul>

				<button type="button" 
					onClick={() => {
						setContacts({ first_name, last_name, phone_number, instagram, snapchat, discord, });
						join();
					}}
				> Join Chat </button>
			</form>
		</div>
    );

}

function setContacts(contactsRef: ContactsRef) {
	
	for(const _field in contactsRef) {

		const field = _field as keyof ContactsRef;

		if(!contactsRef[field].current) {
			continue;
		}

		if(!contactsRef[field].current!.value) {
			continue;
		}

		contacts.value[field] = contactsRef[field].current!.value.replace("@", "");

	}
	
}