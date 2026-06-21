import myReactSinglePageApp from "./src/index.html";

const server = Bun.serve({
	routes: {
		"/": () => new Response("Home")

	}
});
