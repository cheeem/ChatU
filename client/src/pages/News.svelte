<script lang="ts">

    import { useFocus } from "svelte-navigator";

    const focus = useFocus();

    type NewsPost = {
        readonly heading: string,
        readonly subheading?: string,
        readonly content: string,
        date: Date,
    }

    const news: ReadonlyArray<NewsPost> = [
        { 
            heading: "National Science Foundation Innovation Corps Program Grant",
            subheading: "$50,000",
            content: "Our project \"Medevisor: An ML-Based Decision Support System for Analyzing Medical Device History and Recalls\" has received the National Science Foundation (NSF) Innovation Corps (I-Corps) program grant for technology commercialization and customer discovery, $50,000, Sep 2023 to Sep 2024.",
            date: new Date("8/21/2023"),
        },
        { 
            heading: "University of Minnesota Early Innovation Fund",
            subheading: "$10,000",
            content: "Our project \"ML-based decision support system for analyzing medical device history and recalls from FDA's 510k filings\" was awarded an Early Innovation Fund from the University of Minnesota Technology Commercialization.",
            date: new Date("1/9/2023"),
        },
        { 
            heading: "Minnesota Innovation Corps MVP Challenge Grant",
            subheading: "$5,000",
            content: "Our project \"ML-based decision support system for analyzing medical device history and recalls from FDA's 510k filings\" was awarded an MVP Challenge grant from the Minnesota Innovation Corps. This grant will provide seed funding and mentorship for further prototyping and product development.",
            date: new Date("12/23/2022"),
        },
    ] as const;

    const months: ReadonlyArray<string> = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"] as const;
    
    // for(const post of news) {

    //     const obj: any = Object.assign({}, post);

    //     obj.date = obj.date.toISOString().split("T")[0]

    //     fetch("http://localhost:3000/post_news", {
    //         method: "POST",
    //         body: JSON.stringify(obj),
    //         headers: {
    //             //'Content-Type': 'application/x-www-form-urlencoded',
    //             'Content-Type': 'application/json;charset=UTF-8',
    //         }
    //     });

    // }

    // fetch("http://localhost:3000/post_news", {
    //     method: "POST",
    //     body: JSON.stringify({
    //         heading: "kill",
    //         content: "This is a ' string",
    //         date: "2023-04-01"
    //     }),
    //     headers: {
    //         //'Content-Type': 'application/x-www-form-urlencoded',
    //         'Content-Type': 'application/json;charset=UTF-8',
    //     }
    // });

    // async function getNews() {

    //     const res = await fetch("http://localhost:3000/get_news");

    //     const news = await res.json() as NewsPost[];

    //     for(let i = 0; i < news.length; i++) {
    //         news[i].date = new Date(news[i].date);
    //     }

    //     news.sort((a, b) => a.date > b.date ? -1 : 1);

    //     return news;

    // }
        

</script>

<div class="focus" use:focus></div>

<article id="news">
    <div class="heading"> 
        <div class="tab"> </div>
        <h1> News </h1>
    </div>
    <ul> 
        <!-- {#await getNews()}
            <p> loading state </p>
        {:then news} -->
            {#each news as post}
                <li class="date">
                    <p class="month"> {months[post.date.getMonth()]} </p>
                    <p class="day"> {post.date.getDate()} </p>
                    <p class="year"> {post.date.getFullYear()} </p>
                </li>
                <li class="post">
                    <h3> {post.heading} </h3>
                    {#if post.subheading}
                        <h5> {post.subheading} </h5>
                    {/if}
                    <p> {post.content} </p>
                </li>
            {/each}
        <!-- {:catch error}
            <p> {error.message} </p>
        {/await} -->
    </ul>
</article>

<style>

    article {
        --bg-color: var(--neutral2);
        --post-padding: 2.5em;
        --gap: 2em;
        --divider-gap: 0.2em;

        padding: 2.5em var(--side-padding);

        min-height: 100vh;

        background-color: var(--bg-color);
    }

    .heading {
        --tab-width: 0.4em;

        display: flex;
        gap: calc(var(--post-padding) - var(--tab-width));

        margin-bottom: var(--gap);
    }

    .heading .tab {
        width: var(--tab-width);

        background-color: var(--neutral3);
    }

    h1 {
        font-size: 1.5em;
    }

    ul {
        display: grid;
        grid-template-columns: 1fr 7fr;
        gap: var(--gap);
        column-gap: var(--divider-gap);
    }

    li {
        background-color: var(--neutral1);

        font-size: 1em;
    }

    .date {
        display: flex;
        flex-direction: column;
        
        padding: var(--post-padding);
    }

    .date p {
        font-weight: 600;
        font-size: 1.2em;
    }

    .date .year {
        font-weight: 400;
        opacity: 0.8;
    }

    .date .day {
        height: 0.95em;

        font-size: 4.5em;

        translate: 0 -0.125em;
    }

    .post {
        display: grid;
        gap: 0.75em;

        padding: var(--post-padding);
    }

    .post h3 {
        font-size: 1.4em;
    }

    .post h5 {
        font-weight: 600;
        font-size: 1.2em;
    }

    @media only screen and (max-aspect-ratio: 85/100) {
        
        ul {
            grid-template-columns: unset;
            gap: unset;
        }

        .date {
            flex-direction: row;
            gap: 0.3em;

            margin-bottom: var(--divider-gap);
        }

        .post:not(.post:last-of-type) {
            margin-bottom: var(--gap);
        }

        .date .day {
            height: unset;

            font-size: 1.2em;

            translate: unset;
        }

        .date .year {
            font-weight: 600;

            opacity: 1;
        }

        .date .day::after {
            content: ","
        }

        /* .date .year {
            order: 3;
        } */

    }

</style>