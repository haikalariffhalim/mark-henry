#!/usr/bin/env bun


import { $ } from "bun";

const packageJson = (await Bun.file(new URL("../package.json", import.meta.url)).json()) as {
	dependencies?: Record<string, string>;
};
const runtimeExternals = Object.keys(packageJson.dependencies ?? {});

console.log("Building mjJS CLI...");

// Clean dist directory
await $`rm -rf dist`;
await $`mkdir -p dist`;

// Build the CLI using Bun's compile feature if requested
const useCompile = process.argv.includes("--compile");

if (useCompile) {
	console.log(" Creating standalone executable...");

	// Compile to standalone executable
	const compileArgs = [
		"build",
		"./src/cli.ts",
		"--compile",
		"--outfile",
		"./dist",
		"--minify",
	];

	const compileResult = await $`bun ${compileArgs}`;

	if (compileResult.exitCode !== 0) {
		console.error(" Compilation failed");
		process.exit(1);
	}
} else {
	// Traditional build for npm distribution
	const result = await Bun.build({
		entrypoints: ["./src"],
		outdir: "./dist",
		target: "bun",
		format: "esm",
		minify: true,
		external: runtimeExternals,
	});

	if (!result.success) {
		console.error("Build failed:", result.logs);
		process.exit(1);
	}


	const jsContent = await Bun.file("main.mjs").text();
	// Only add shebang if it doesn't already have one
	const finalContent = jsContent.startsWith("#!")
		? jsContent
		: `#!/usr/bin/env bun\n${jsContent}`;
	await Bun.write("./main.mjs", finalContent);
	await $`chmod +x main.mjs`;
}

// Build library
const libResult = await Bun.build({
	entrypoints: ["./src/index.ts"],
	outdir: "./dist",
	target: "bun",
	format: "esm",
	minify: false,
	external: runtimeExternals,
});

if (!libResult.success) {
	console.error("Library build failed:", libResult.logs);
	process.exit(1);
}

console.log(" Build complete!");

// Show build stats
const stats = await $`du -sh dist`.text();
console.log(` Output size: ${stats.trim()}`);
