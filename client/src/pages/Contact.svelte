<script lang="ts">

</script>

<article id="contact"> 
    <form>
        <h1> Contact <span>Us</span> </h1>
        <ul> 
            <li> 
                <input id="contact-name" type="text" name="name" placeholder=" " />
                <label for="contac-name"> Name </label>
            </li>
            <li> 
                <input id="contact-email" type="email" name="email" placeholder=" " />
                <label for="contact-email"> Email </label>
            </li>
            <li> 
                <textarea id="contact-message" name="message" placeholder=" "></textarea>
                <label for="contact-message"> Your Message </label>
            </li>
        </ul>
        <button type="submit">
            Send
        </button>
    </form>
</article>

<style>

    article {
        padding: 2.5em var(--side-padding);

        background-color: var(--neutral1);
    }

    form {
        --form-gap: 1.5em;
        --field-padding: 0.5em;
        --field-border: 0.15em;

        display: grid;
        gap: var(--form-gap);
    }

    form h1 {
        font-size: 1.5em;
    }

    form ul {
        display: grid; 
        gap: var(--form-gap);
    }

    form ul li {
        position: relative;
        display: flex;
        align-items: center;
        gap: 0.5em;
    }

    textarea {
        height: 30vh;
    }

    input, textarea {
        padding: var(--field-padding);

        font-size: 1em;

        background-color: var(--neutral1);
        
        border: var(--field-border) solid var(--neutral3);

        outline: none;
    }

    input, textarea {
        width: 100%;
    }

    input ~ label, textarea ~ label {
        position: absolute;
        top: -0.7em;
        left: calc(var(--field-padding) + var(--field-border));

        background-color: var(--neutral1);

        font-weight: 600;

        pointer-events: none;

        transition: top 0.2s ease;
    }

    input:placeholder-shown ~ label, textarea:placeholder-shown ~ label {
        top: calc(var(--field-padding) + var(--field-border));
    }

    input:focus ~ label, textarea:focus ~ label {
        top: -0.7em;
    }
    
</style>