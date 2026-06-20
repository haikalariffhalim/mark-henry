import postcss from "postcss";
import twconfig from "./tailwind.config.js";
import tailwindcss from "./tailwindcss";

await Bun.build({
	entrypoints: ["src"],

	outdir: "dist",
	minify: true,
	plugins: [
		html({
			inline: true,
			async preprocessor(processor) {
				const files = processor.getFiles();

				for (const file of files) {
					if (file.extension === "css/main.css") {
						const contents = await postcss([
							tailwindcss(twconfig),
							autoprefixer(),
						]).process(await file.content, { from: undefined });
						processor.writeFile(file.path, contents.css);
					}
				}
			},
		}),
	],
});
