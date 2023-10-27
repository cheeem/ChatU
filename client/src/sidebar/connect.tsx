import "./connect.css";

import { connectState, contacts } from '../app';
import { sendEvent } from "../websocket";

export default function Connect() {

    const connectClass = `connect-${connectState}`;
    
    if(connectState.value === "new") {        
        return (
            <aside id="connect" class={connectClass}> 
                <h3> Like who you're talking to? Make a connection! </h3>
                <ConnectForm />
            </aside>
        );
    }

    if(connectState.value === "failure") {
        return (
            <aside id="connect" class={connectClass}> 
                <h3> Looks like the connection didn't work out... Do you want to try again? </h3>
                <ConnectForm />
            </aside>
        );
    }

    if(connectState.value === "request") {
        return ( 
            <aside id="connect" class={connectClass}> 
                <h3> Another user wants to connect!  </h3>
                <ConnectForm />
                <button> Decline Connection </button>
            </aside>
        );
    }

    if(connectState.value === "sent") {
        return (
            <aside id="connect" class={connectClass}> 
                <h3> Connection sent. Would you like to cancel? </h3>
                <button> Cancel Connection </button>
            </aside>
        );
    }

    if(connectState.value === "success") {
        return (
            <aside id="connect" class={connectClass}> 
                <h3> Successfully Connected! </h3>
                <ul id="new-connections">
                    <li> 
                        <ul class="connection-content">
                            <li> 
                                <p> insta </p>
                                <p> @dfjvkjdhfk </p>
                            </li>
                        </ul>
                    </li>
                </ul>
            </aside>
        );
    } 
    
    return (
        <aside id="connect" class={connectClass}> 
            <h3> Like who you're talking to? Make a connection! </h3>
            <ConnectForm />
        </aside>
    );

}

function ConnectForm() {

    console.log(contacts.value);

    return (
        <form>
            <p> Select your contacts to share: </p>
            <ul>
                {contacts.value.first_name ? <ConnectField value={contacts.value.first_name!}/> : <></>}
                {contacts.value.last_name ? <ConnectField value={contacts.value.last_name!}/> : <></>}
                {contacts.value.phone_number ? <ConnectField value={contacts.value.phone_number!}/> : <></>}
                {contacts.value.instagram ? <ConnectField value={contacts.value.instagram!}/> : <></>}
                {contacts.value.snapchat ? <ConnectField value={contacts.value.snapchat!}/> : <></>}
                {contacts.value.discord ? <ConnectField value={contacts.value.discord!}/> : <></>}
            </ul>
            <button onClick={() => sendEvent("Connect")}> Connect </button>
        </form>
    );

}

function ConnectField(props: { value: string }) {
    
    return (
        <li> 
            <input type="checkbox" />
            <label> {props.value} </label>
        </li>
    );

}