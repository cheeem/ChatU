<script lang="ts">
   
    import { onMount } from "svelte";
    import { useFocus } from "svelte-navigator";

    import img_hero from '../img/hero.png';
    import img_hero_outline from '../img/hero_outline.png';
    import img_display_1 from '../img/testalize-me-SVmaaACzcJ8-unsplash.jpg';
    import img_display_2 from '../img/national-cancer-institute-2fyeLhUeYpg-unsplash.jpg';
    import img_display_3 from '../img/jair-lazaro-0lrJo37r6Nk-unsplash.jpg';
    import img_display_4 from '../img/arseny-togulev-DE6rYp1nAho-unsplash.jpg';

    const focus = useFocus();

    const images: string[] = [
        img_display_1,
        img_display_2,
        img_display_3,
        img_display_4
    ];

    const intersectionObserver: IntersectionObserver = new IntersectionObserver((entries: IntersectionObserverEntry[], observer: IntersectionObserver) => {        
        entries.forEach(entry => {
            if(!entry.isIntersecting) return;
            entry.target.classList.add("view");
            observer.unobserve(entry.target);
        });
    });

    let facts: HTMLUListElement;

    let previous: number = 0
    let current: number = 1;

    onMount(() => {
        intersectionObserver.observe(facts);
    });
    
    setInterval(() => {
        previous = current % images.length;
        current = (current + 1) % images.length;
    }, 5000);


</script>

<div class="focus" use:focus></div>

<article id="home">

    <div class="hero">
        <div class="image">
            <img src={img_hero_outline} alt="" />
            <img src={img_hero} alt="" />
        </div>
        <div class="content"> 
            <h1> <span class="spanalt">Augments</span> human evaluation of medical device safety with <span class="spanalt">advanced</span> data analytics. </h1> 
            <div class="cta">
                <a href="#introduction"> <button> Learn More </button> </a>
                <!-- <a href="https://system.medevisor.com" target="_blank" rel="noopener noreferrer"> <button> Access Medevisor </button> </a> -->
            </div>
        </div>
    </div>

    <section id="introduction" class="what">

        <div class="content">
            
            <h1> What is Medevisor? </h1>

            <div class="description">

                <p> 
                    Medevisor is an <span>online decision support system</span> that provides solutions to help medical device manufacturers and regulators timely and accurately <span>predict medical device adverse events</span> based on machine learning and artificial intelligence.
                </p>
    
                <p>
                    You pick a device, we let you know its safety evaluation and <span>facilitate your data-driven decision-making!</span>
                </p>
    
                <a href="https://system.medevisor.com" target="_blank" rel="noopener noreferrer"> <button>See Medevisor For Yourself</button> </a>

            </div>

        </div>

        <div class="image-wrapper">
            <div class="image">
                <img src={images[previous]} alt="" /> 
                {#key current} <img src={images[current]} alt="" /> {/key}
            </div>
        </div>

    </section>

    <section class="know">
        
        <div class="content"> 

            <h1> Did you know? </h1>

            <ul bind:this={facts}> 

                <li>
                    <h3> 90% </h3>
                    <p> of <span class="spanalt">medical devices</span> in the US do not provide human clinical test data for approval. </p>
                </li>
                <li> 
                    <h3> 300+ </h3>
                    <p> <span class="spanalt">medical devices</span> are recalled annually on average. </p>
                </li>
                <li> 
                    <h3> 430M+ </h3>
                    <p> <span class="spanalt">units</span> are recalled annually on average. </p>
                </li>
                <li> 
                    <h3> 8000+ </h3>
                    <p> <span class="spanalt">deaths</span> are directly related to medical device recalls annually on average. </p>
                </li>
                <li> 
                    <h3> $2.5-5B </h3>
                    <p> is the <span class="spanalt">direct cost</span> of medical device recall to the whole industry annually on average. </p>
                </li>

            </ul>

        </div>

    </section>

    <section class="solution">
        
        <div class="content"> 
            <h1> An End-to-end Solution </h1>
            <p> 
                Medevisor's end-to-end system provides solutions for medical device adverse events using
                advanced data analytics. The system solutions follows a three-step process: Analyze the past,
                Showcase the present, and Predict future (ASP).
            </p>
        </div>

    </section>
    
</article>

<style>

    section {
        padding: max(10vw, 7.5vh) var(--side-padding);
    }

    section .content {
        display: grid;
        gap: 1.2em; 

        font-size: 1.3em;
    }

    section h1 {
        font-size: 1.5em;
    }

    .hero {
        --hero-color: var(--neutral4);
        --hero-txt-color: var(--neutral1);

        overflow: hidden;

        position: relative;

        width: 100%;
        height: 50vh;

        background-color: var(--hero-color);
    }

    .hero span {
        color: var(--hero-txt-color);
    }

    .hero .image {
        overflow: hidden;

        position: absolute;
        right: var(--side-padding);
        bottom: 0;

        height: 80%;  

        animation: 1.25s ease 0s 1 slide-bottom;
        animation-fill-mode: backwards;
        animation-timing-function: ease;
    }

    .hero .image img {
        height: 100%;
    }

    .hero .image img:nth-of-type(1) {
        position: absolute;
        top: 0;
        left: 0;
    }

    .hero .image img:nth-of-type(2)  {
        translate: 1.5% 1.5%;
    }

    .hero .content {
        position: absolute;
        top: 10%;
        left: 0;

        display: flex;
        flex-direction: column;
        gap: 1em;

        font-size: 1.3em;

        padding: 0 var(--side-padding);

        width: 60%;

        animation: 0.75s ease 0s 1 slide-left;
        animation-fill-mode: backwards;
    }

    .hero h1 {
        font-size: 1.6em;
        color: var(--hero-txt-color);
    }

    .hero .cta {
        display: flex;
        gap: 1em;
    }

    .hero button {
        --btn-main-color: var(--hero-txt-color);
        --btn-alt-color: var(--hero-color);
        font-size: 0.7em;
    }

    .what {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 10%;
    }

    .what .description {
        display: grid;
        gap: 1.5em;
    }

    .what a {
        width: fit-content;
    }

    .what button {
        font-size: 0.7em;
    }

    .what .image-wrapper {
        position: relative;

        width: 100em;
        height: 21.5em;

        translate: 0 1em;
    }

    .what .image-wrapper::before {
        position: absolute;
        top: -1em;
        left: -1.25em;

        height: 100%;
        width: 100%;

        background-color: var(--neutral3);
        
        content: "";
    }

    .what .image {
        overflow: hidden;
        position: relative;

        width: 100%;
        height: 100%;
    }

    .what img {
        height: 100%;
        width: 100%;

        object-fit: cover;
    }

    .what img:nth-of-type(2) {
        position: absolute;
        top: 0;
        left: 0;

        animation: 1s ease 0s 1 slide-left;
        animation-fill-mode: backwards;
    }

    .know {
        background-color: var(--neutral3);
    }

    .know * {
        color: var(--neutral1);
    }

    .know ul {
        display: flex;
        flex-wrap: wrap;
        gap: 1em;

        margin-top: 1em;
    }

    .know li {
        flex: 1 1 17.5em;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5em;

        padding: 1em 2.5em 1.5em 2.5em;

        background-color: var(--neutral4);

        opacity: 0;
    }

    .know h3 {
        font-size: 3em;
    }

    .know p {
        text-align: center;
    }

    .solution {
        background-color: var(--neutral4);
    }

    .solution * {
        color: var(--neutral1);
    }

    @media only screen and (max-aspect-ratio: 85/100) {

        .hero .content {
            top: 7.5vw;

            font-size: 1.3em;

            width: 100%;
        }

        .hero .image {
            height: 27.5vh;
        }

        .what {
            flex-direction: column;
            gap: 3vh;
        }

        .what .image-wrapper {
            width: 100%;

            scale: 0.95;
        }

    }
    
    @keyframes slide-bottom {
        0%, 66% {
            translate: 0 100%;
        }
        100% {
            translate: 0 0;
        }
    }

    @keyframes slide-left {
        0% {
            translate: -150% 0;
        }
        100% {
            translate: 0 0;
        }
    }

</style>