type Context = { input: string; files: Record<string, string> };
type Language = {
	name: string;
	aliases: string[];
	run?: (ctx: Context) => void;
};
export const languages: Language[] = [
	{
		name: "JavaScript",
		aliases: ["js", "javascript", "jsx", "cjs", "mjs"],
		run(ctx) {
			ctx.files.script += ctx.input + "\n";
		},
	},
	{
		name: "Shell",
		aliases: ["sh", "bash", "zsh", "shell"],
		run(ctx) {
			ctx.files.script +=
				ctx.input
					.split("\n")
					.filter(
						(line) => line.trim().length > 0 && !line.trim().startsWith("#"),
					)
					.map((line) => `await $\`${line}\`\n`)
					.join("\n") + "\n";
		},
	},
	{
		name: "TypeScript",
		aliases: ["ts", "tsx", "typescript", "mts", "cts"],
		run(ctx) {
			ctx.files.script += ctx.input + "\n";
		},
	},
];
