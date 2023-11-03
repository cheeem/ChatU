<script lang="ts">
    import type { ComponentType } from "svelte";
    import { Router, Route, Link } from "svelte-navigator";

	import Home from "./pages/Home.svelte";
	import News from "./pages/News.svelte";
	import Contact from "./pages/Contact.svelte";
	import About from "./pages/About.svelte";

	type Route = {
		path: string
		name: string
		page: ComponentType
	}

	const routes: Route[] = [
		{
			path: "/",
			name: "Home",
			page: Home,
		},
		{
			path: "/news",
			name: "News",
			page: News,
		},
		{
			path: "/contact",
			name: "Contact",
			page: Contact,
		},
		{
			path: "/about",
			name: "About",
			page: About,
		},
	];

	let menuOpen: boolean = false;

</script>

<Router>
	
	<nav class={menuOpen ? "nav-menu-open" : ""}>
		<div id="nav-mobile">
			<div class="logo"> <Link to="/"> MEDEVISOR </Link> </div>
			<div id="menu-wrapper">
				<svg id="menu" viewBox="0 0 100 100" width="80" tabindex="0" role="button" aria-pressed={menuOpen} 
					on:click={() => menuOpen = !menuOpen} 
					on:keydown={() => menuOpen = !menuOpen}
				>
					<path class="line top" d="m 70,33 h -40 c 0,0 -8.5,-0.149796 -8.5,8.5 0,8.649796 8.5,8.5 8.5,8.5 h 20 v -20" />
					<path class="line middle" d="m 70,50 h -40" />
					<path class="line bottom" d="m 30,67 h 40 c 0,0 8.5,0.149796 8.5,-8.5 0,-8.649796 -8.5,-8.5 -8.5,-8.5 h -20 v 20" />
				</svg>
			</div>
		</div>
		<ul>
			{#each routes as route}
				<li> 
					<Link to={route.path} 
						on:click={() => menuOpen = false} 
						on:keydown={() => menuOpen = false}
					> 
						{route.name} 
					</Link> 
				</li>
			{/each}
		</ul>
		<a class="to-system" href="https://system.medevisor.com" target="_blank" rel="noopener noreferrer"> <button id="login"> Access Medevisor </button> </a>
	</nav>

	<main>
		{#each routes as route}
			<Route path={route.path}> <svelte:component this={route.page} /> </Route>
		{/each}
	</main>

	<footer>

		<div class="footer-wrapper">

			<div class="logo"> <Link to="/"> MEDEVISOR </Link> </div>

			<div class="divider"></div>

			<div class="links"> 

				<ul>
					{#each routes as route}
						<li> <Link to={route.path}> {route.name} </Link> </li>
					{/each}
				</ul>

				<!-- <ul> 
					socials
				</ul> -->

			</div>

		</div>

	</footer>

</Router>
