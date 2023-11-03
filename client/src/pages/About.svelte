<script lang="ts">
    import { useFocus } from "svelte-navigator";

    import img_yizhu from "../img/YiZhu.png";
    import img_soumyasen from "../img/SoumyaSen.png";
    import img_karacamandic from "../img/karacamandic.png"

    const focus = useFocus();

    type Field = "Information & Decision Sciences" | "Finance";//| "Economics" | "Mathematics"; 

    type Contributor = {
        readonly link: string
        readonly image: string
        readonly name: string
        readonly titles: ReadonlyArray<string>
        readonly field: Field 
    }

    const contributors: ReadonlyArray<Contributor> = [
        {
            link: "https://carlsonschool.umn.edu/faculty/yizhu",
            name: "Yi Zhu",
            image: img_yizhu,
            titles: ["PhD Candidate"] as const,
            field: "Information & Decision Sciences",
        },
        {
            link: "https://carlsonschool.umn.edu/faculty/soumya-sen",
            name: "Soumya Sen",
            image: img_soumyasen,
            titles: ["Associate Professor", "3M Fellow", "Mcknight Presidential Fellow"] as const,
            field: "Information & Decision Sciences",
        },
        {
            link: "https://carlsonschool.umn.edu/faculty/pinar-karaca-mandic",
            name: "Pinar Karaca-Mandic",
            image: img_karacamandic,
            titles: ["Distinguished Mcknight University Professor", "C. Arthur Williams Jr. Professor In Healthcare Risk Management"] as const,
            field: "Finance",
        },
    ] as const;

</script>

<div class="focus" use:focus></div>

<article id="about">
    <h1> Meet The <span>Medevisor Team</span> </h1>
    <ul class="contributors">
        {#each contributors as contributer}
            <li> 
                <a href={contributer.link} target="_blank" rel="noopener noreferrer">
                    <div class="image"> 
                        <img src={contributer.image} alt={contributer.name} />
                    </div>
                    <div class="content">
                        <h3 class="name"> {contributer.name} </h3>
                        <p class="field"> {contributer.field} </p>
                        <ul class="titles"> 
                            {#each contributer.titles as title}
                                <li> {title} </li>
                            {/each}
                        </ul>
                    </div>
                </a>
            </li>
        {/each}
    </ul>
</article>

<style>

    article {
        padding: 2.5em var(--side-padding);

        height: 100%;
    }

    h1 {
        margin-bottom: 1.5em;

        font-size: 1.5em;
    }

    .contributors { 
        display: flex;
        flex-wrap: wrap;
        gap: 3em;

        height: 100%;
    }

    .contributors > li:nth-of-type(1) {
        animation: 1s ease 0s 1 fade-in;
        animation-fill-mode: backwards;
    }

    .contributors > li:nth-of-type(2) {
        animation: 1s ease 0.2s 1 fade-in;
        animation-fill-mode: backwards;
    }

    .contributors > li:nth-of-type(3) {
        animation: 1s ease 0.4s 1 fade-in;
        animation-fill-mode: backwards;
    }

    .contributors > li {
        flex: 17.5em 1 1;
    }

    .contributors > li > a {
        height: 100%;

        cursor: pointer;
    }

    .contributors > li > a > .image {
        overflow: hidden;

        position: relative;

        margin-bottom: 1em;

        width: 100%;
        
        aspect-ratio: 1;
    }

    .contributors li > a > .image > img {
        width: 100%;
        height: 100%;

        transition: scale 0.4s ease;
    }

    .contributors li > a > .image::after {
        position: absolute;
        top: 0;
        left: 0;

        width: 100%;
        height: 100%;

        content: "";

        transition: background-color 0.5s ease;

        opacity: 0.1;
    }

    .contributors li > a:hover > .image > img {
        scale: 1.02;
    }

    .contributors li > a:hover > .image::after {
        background-color: var(--neutral3);
    }

    .contributors > li > a > .content {
        display: grid;
        gap: 0.3em;
    }

    .contributors > li > a > .content > .name {
        font-size: 1.5em;
    }

    .contributors > li > a > .content > .field {
        font-weight: 600;
    }

    @keyframes fade-in {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 100%;
        }
    }

</style>