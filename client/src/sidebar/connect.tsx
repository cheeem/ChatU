import "./connect.css";

import { /*MutableRef,*/ useRef } from "preact/hooks";

import { connectState, connectContacts, contacts, x500 } from '../app';
import { UserContacts } from "../contacts";
import { sendEvent } from "../websocket";

// type ContactsRef = {
// 	first_name: MutableRef<null | HTMLInputElement>,
// 	last_name: MutableRef<null | HTMLInputElement>,
// 	phone_number: MutableRef<null | HTMLInputElement>,
// 	instagram: MutableRef<null | HTMLInputElement>,
// 	snapchat: MutableRef<null | HTMLInputElement>,
// 	discord: MutableRef<null | HTMLInputElement>,
// }

export default function Connect() {

    const first_name = useRef<null | HTMLInputElement>(null);
	const last_name = useRef<null | HTMLInputElement>(null);
	const phone_number = useRef<null | HTMLInputElement>(null);
	const instagram = useRef<null | HTMLInputElement>(null);
	const snapchat = useRef<null | HTMLInputElement>(null);
	const discord = useRef<null | HTMLInputElement>(null);

    const connectClass = `connect-${connectState}`;
    
    if(connectState.value === "new" || connectState.value === "failure") {        
        return (
            <aside id="connect" class={connectClass}> 
                <h3> Like who you're talking to? Make a connection! </h3>
                <form onSubmit={(e: Event) => e.preventDefault()}>
                    <p> Select your contacts to share: </p>
                    <ul>
                        {contacts.value.first_name ? ( <li> <input type="checkbox" ref={first_name} /> <label> {contacts.value.first_name} </label> </li> ) : <></>}
                        {contacts.value.last_name ? ( <li> <input type="checkbox" ref={last_name} /> <label> {contacts.value.last_name} </label> </li> ) : <></>}
                        {contacts.value.phone_number ? ( <li> <input type="checkbox" ref={phone_number} /> <label> {contacts.value.phone_number} </label> </li> ) : <></>}
                        {contacts.value.instagram ? ( <li> <input type="checkbox" ref={instagram} /> <label> {contacts.value.instagram} </label> </li> ) : <></>}
                        {contacts.value.snapchat ? ( <li> <input type="checkbox" ref={snapchat} /> <label> {contacts.value.snapchat} </label> </li> ) : <></>}
                        {contacts.value.discord ? ( <li> <input type="checkbox" ref={discord} /> <label> {contacts.value.discord} </label> </li> ) : <></>}
                    </ul>
                </form>
                <button onClick={() => {
                    const payload = new Uint8Array(6);

                    if(contacts.value.first_name) payload[0] = +(first_name.current!.checked);
                    if(contacts.value.last_name) payload[1] = +(last_name.current!.checked);
                    if(contacts.value.phone_number) payload[2] = +(phone_number.current!.checked);
                    if(contacts.value.instagram) payload[3] = +(instagram.current!.checked);
                    if(contacts.value.snapchat) payload[4] = +(snapchat.current!.checked);
                    if(contacts.value.discord) payload[5] = +(discord.current!.checked);
    
                    //console.log(payload)
                    sendEvent("Connect", payload);
                    connectState.value = "sent";
                }}> Connect </button>
            </aside>
        );
    }

    // if(connectState.value === "failure") {
    //     return (
    //         <aside id="connect" class={connectClass}> 
    //             <h3> Looks like the connection didn't work out... Do you want to try again? </h3>
    //             <ConnectForm />
    //             <button onClick={() => {
    //                 sendEvent("Connect");
    //                 connectState.value = "sent";
    //             }}> Connect </button>
    //         </aside>
    //     );
    // }

    if(connectState.value === "request") {
        return ( 
            <aside id="connect" class={connectClass}> 
                <h3> Another user wants to connect!  </h3>
                <form onSubmit={(e: Event) => e.preventDefault()}>
                    <p> Select your contacts to share: </p>
                    <ul>
                        {contacts.value.first_name ? ( <li> <input type="checkbox" ref={first_name} /> <label> {contacts.value.first_name} </label> </li> ) : <></>}
                        {contacts.value.last_name ? ( <li> <input type="checkbox" ref={last_name} /> <label> {contacts.value.last_name} </label> </li> ) : <></>}
                        {contacts.value.phone_number ? ( <li> <input type="checkbox" ref={phone_number} /> <label> {contacts.value.phone_number} </label> </li> ) : <></>}
                        {contacts.value.instagram ? ( <li> <input type="checkbox" ref={instagram} /> <label> {contacts.value.instagram} </label> </li> ) : <></>}
                        {contacts.value.snapchat ? ( <li> <input type="checkbox" ref={snapchat} /> <label> {contacts.value.snapchat} </label> </li> ) : <></>}
                        {contacts.value.discord ? ( <li> <input type="checkbox" ref={discord} /> <label> {contacts.value.discord} </label> </li> ) : <></>}
                    </ul>
                </form>
                <button onClick={() => {
                    const payload = new Uint8Array(6);

                    if(contacts.value.first_name) payload[0] = +(first_name.current!.checked);
                    if(contacts.value.last_name) payload[1] = +(last_name.current!.checked);
                    if(contacts.value.phone_number) payload[2] = +(phone_number.current!.checked);
                    if(contacts.value.instagram) payload[3] = +(instagram.current!.checked);
                    if(contacts.value.snapchat) payload[4] = +(snapchat.current!.checked);
                    if(contacts.value.discord) payload[5] = +(discord.current!.checked);
    
                    //console.log(payload)
                    sendEvent("Connect", payload);
                    connectState.value = "sent";
                }}> Connect </button>
                <button onClick={() => {
                    sendEvent("ConnectCancel");
                }}> Decline Connection </button>
            </aside>
        );
    }

    if(connectState.value === "sent") {
        return (
            <aside id="connect" class={connectClass}> 
                <h3> Connection sent. Would you like to cancel? </h3>
                <button onClick={() => {
                    sendEvent("ConnectCancel");
                }}> Cancel Connection </button>
            </aside>
        );
    }

    if(connectState.value === "success") {
        return (
            <aside id="connect" class={connectClass}> 
                <h3> Successfully Connected! </h3>
                <ul id="new-connections">
                    {connectContacts.value ? connectContacts.value.map((contacts: UserContacts) => contacts.x500 === x500 ? <> </> : (
                        <li> 
                            <ul class="connection-content">
                                {contacts.first_name ? ( <li> <p> First Name </p> <p> {contacts.first_name} </p> </li> ) : <></>}
                                {contacts.last_name ? ( <li> <p> Last Name </p> <p> {contacts.last_name} </p> </li> ) : <></>}
                                {contacts.phone_number ? ( <li> <p> Phone Number </p> <p> {contacts.phone_number} </p> </li> ) : <></>}
                                {contacts.instagram ? ( <li> <p> Instagram </p> <p> {contacts.instagram} </p> </li> ) : <></>}
                                {contacts.snapchat ? ( <li> <p> Snapchat </p> <p> {contacts.snapchat} </p> </li> ) : <></>}
                                {contacts.discord ? ( <li> <p> First Name </p> <p> {contacts.discord} </p> </li> ) : <></>}
                            </ul>
                        </li>
                    )) : <></>}
                </ul>
            </aside>
        );
    } 
    
    return (
        <aside id="connect" class={connectClass}> 
            <h3> Like who you're talking to? Make a connection! </h3>
            <form onSubmit={(e: Event) => e.preventDefault()}>
                <p> Select your contacts to share: </p>
                <ul>
                    {contacts.value.first_name ? ( <li> <input type="checkbox" ref={first_name} /> <label> {contacts.value.first_name} </label> </li> ) : <></>}
                    {contacts.value.last_name ? ( <li> <input type="checkbox" ref={last_name} /> <label> {contacts.value.last_name} </label> </li> ) : <></>}
                    {contacts.value.phone_number ? ( <li> <input type="checkbox" ref={phone_number} /> <label> {contacts.value.phone_number} </label> </li> ) : <></>}
                    {contacts.value.instagram ? ( <li> <input type="checkbox" ref={instagram} /> <label> {contacts.value.instagram} </label> </li> ) : <></>}
                    {contacts.value.snapchat ? ( <li> <input type="checkbox" ref={snapchat} /> <label> {contacts.value.snapchat} </label> </li> ) : <></>}
                    {contacts.value.discord ? ( <li> <input type="checkbox" ref={discord} /> <label> {contacts.value.discord} </label> </li> ) : <></>}
                </ul>
            </form>
            <button onClick={() => {

                const payload = new Uint8Array(6);

                if(contacts.value.first_name) payload[0] = +(first_name.current!.checked);
                if(contacts.value.last_name) payload[1] = +(last_name.current!.checked);
                if(contacts.value.phone_number) payload[2] = +(phone_number.current!.checked);
                if(contacts.value.instagram) payload[3] = +(instagram.current!.checked);
                if(contacts.value.snapchat) payload[4] = +(snapchat.current!.checked);
                if(contacts.value.discord) payload[5] = +(discord.current!.checked);

                //console.log(payload)

                sendEvent("Connect", payload);

                connectState.value = "sent";

            }}> Connect </button>
        </aside>
    );

}