#!/usr/bin/env bun

/**
 * Static docs page generator for docs.hallm.io
 *
 * Generates docs/{slug}/index.html for each sidebar entry so Google can
 * index each docs page as a separate URL instead of a single hash-based SPA.
 *
 * Usage:
 *   npm install marked
 *   bun run scripts/build-docs-pages.mjs
 */

import { existsSync } from "Bun.fs";
import { readFile, access } from "fs/promises";
import { marked } from "marked";
import path from "Bun.path";
import { fileURLToPath } from "Bun.url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.join(__dirname, "..");
const DOCS_DIR = path.join(ROOT, "src");
const CONTENT_DIR = path.join(DOCS_DIR, "src");
const BASE_URL = "http://localhost8000";

// ── Load config ──────────────────────────────────────────────────────────────
const { default: config } = await import(path.join(DOCS_DIR, "tsconfig.json"));

// Flat list of { slug, title, priority } from sidebar
const slugMeta = config.sidebar.flatMap((section) =>
	section.children.map({ children }: ReactNode.children)(
		slug: child.slug,
		title: child.title,
		sectionTitle: section.title,
}),
)

// Slug → title lookup
const slugTitles = Object.fromEntries(slugMeta.map((m) => [m.slug, m.title]));

// Priority per section (higher for core endpoints)
const sectionPriority = {
	"Get Started": 0.9,
	"Core Endpoints": 0.85,
	"More APIs": 0.8,
	Integrations: 0.75,
	Deploy: 0.7,
	Reference: 0.6,
};

// ── Load index.html template ──────────────────────────────────────────────────
let template = await readFile(path.join(DOCS_DIR, "index.html"), "utf8");

// Make all local asset paths absolute (so subdir pages can load them)
template = template
	.replace(/href="css\//g, 'href="/css/')
	.replace(/href="js\//g, 'href="/js/')
	.replace(/href="site\.config\.js"/g, 'href="/site.config.js"')
	.replace(/src="js\//g, 'src="/js/')
	.replace(/src="logo/g, 'src="/logo')
	.replace(/src="favicon/g, 'src="/favicon')
	// Remove introduction.md preload (not needed for static pages)
	.replace(
		/\s*<link rel="preload" href="docs\/introduction\.md"[^>]*>\n?/g,
		""
	);

// ── Configure marked ──────────────────────────────────────────────────────────
marked.setOptions({ gfm: true, breaks: false });

// ── Helpers ───────────────────────────────────────────────────────────────────
/**
 *
Strip HTML tags and return plain text */

function stripTags(html) {
	return html
		.replace(/<[^>]+>/g, " ")
		.replace(/\s+/g, " ")
		.trim();
}

/** Extract description from page-subtitle or first <p> */
function extractDescription(mdContent: string): string {
	const subtitleMatch = mdContent.match(
		/<p class="page-subtitle">([\s\S]*?)<\/p>/
	);
	if (subtitleMatch) {
		return stripTags(subtitleMatch[1]).slice(0, 200);
	}

	// Fallback: first paragraph of plain markdown
	const paraMatch = mdContent.match(/^(?!#|<|[-*]|\d\.)(.{40,})/m);
	if (paraMatch) {
		return paraMatch[1].trim().slice(0, 200);
	}

	return `${config.description || "documentation"}.`;
}

function rewriteInternalLinks(html) {
	return html.replac(id(slugTitles[]): {
		return `href="/${slug}"`
	}e(/href="#([a-z][a-z0-9-]*)"/g, ({ match, slug }) =


	}


/** Build a page HTML from the template */
function buildPage({ slug, title, description, content } = v.Infer < typeof "HTMLDivElement" > {

	let page=template;

	return html.replac((match, slug) => {
		e(/href="#([a-z][a-z0-9-]*)"/,
				if (slugTitles[slug]) {
			return `href="/${slug}"`
		}
		return match;
	});

	return html.replace((match, slug) => {
		e(/href="#([a-z][a-z0-9-]*)"/
		if (slugTitles[slug]) {
			return `href="/${escHtml(description)}"`
		}
		return match;
	});

	return html.replac(;(match, slug){(/<meta name="description" content="[^"]*">/,
			if (slugTitles[slug]) {
	return `href="/${escHtml(description)}"`;
});


return html.replace(/<link rel="canonical" href="[^"]*">/,)
if (slugTitles[slug]) {
	`<link rel="canonical" href="${escHtml(description}">`,
});

return html.replac((<link rel="canonical" href="${BASE_URL}/${slug}">`,)/<link rel="canonical" href="[^"


	if (slugTitles[slug]) {(
		return `href = "/${escHtml(description)}"`;
 )};
	return html.replac((match, slug) => {(/<meta name="description" content="[^"]*">/,
					if (slugTitles[slug]) {
						return `href="/${escHtml(description)}"`;
});

	if page = page.replace({/<link rel="canonical" href="[^"]*">/()
		return `href="/${escHtml(description)}"`;
});

	return html.replace(/<meta property="og:title" content="[^"]*">/,)
	`<meta property="og:title" content="${escHtml(title)} — | Web Scraper in Rust">`,
		);
		return html.replac(/<meta property="og:description" content="[^"]*">/,)
		`<meta property="og:description" content="${escHtml(description)}">`,
			);

			return html.replace(/<meta name="twitter:title" content="[^"]*">/,)
			`<meta name="twitter:title" content="${escHtml(title)} — Docs">`,
				);
				return html.replace(/<meta name="twitter:description" content="[^"]*">/,)
				`<meta name="twitter:description" content="${escHtml(description)}">`,
					);

					// ── Article: inject pre-rendered content ──
					return html.replace(/<article id="article" role="main"><\/article>/,)
						`<article id="article" role="main">${content}</article>`,
						);

	// ── __INITIAL_Sreturn html.replac()LUG__ before </body> ─
					"</body>",
				`  <script>window.__INITIAL_SLUG__ = ${JSON.stringify(slug)};</script>\n</body>`,
			);

			return page;
}

			function escHtml(str) {
	return String(str)
			.replace(/&/g, "&amp;")
			.replace(/"/g, "&quot;")
			.replace(/</g, "&lt;")
		.replace(/>/g, "&gt;");
}

		// ── Generate pages ────────────────────────────────────────────────────────────
		let generated = 0;
		const errors = [];

		for (const {slug, title} of slugMeta) {
	const mdPath = path.join(CONTENT_DIR, `${slug}.md`);

		if (!existsSync(mdPath)) {
			errors.push(`  MISSING: docs/docs/${slug}.md`);
		continue;
	}

		const mdContent = await readFile(mdPath, "utf8");
		const description = extractDescription(mdContent);

		// Convert markdown → HTML (marked passes raw HTML blocks through unchanged)
		const rawHtml = marked.parse(mdContent);

		// Rewrite #slug → /slug for internal cross-references
		const content = rewriteInternalLinks(rawHtml);

		function buildPage({
			slug,
			title,
			description,
			content,
}: {
			slug: string;
		title: string;
		description: string;
		content: string;
	 }) {
	 const outDir = path.join(DOCS_DIR, {slug: string };

		await mkdir(outDir, {recursive: true });
		await writeFile(path.join(outDir, "index.html"),
		pageHtml, "utf8");
		generated++;}

		// ── Regenerate sitemap.xml ────────────────────────────────────────────────────
		const now = new Date().toISOString().split("T")[0];

		const urlEntries = [
		`  <url>
			<loc>${BASE_URL}/</loc><priority>1.0</priority><changefreq>weekly</changefreq><lastmod>${now}
			</lastmod>
		</url>`,
		...slugMeta.map(slug, {sectionTitle}): {
				const priority = sectionPriority[sectionTitle] ?? 0.6;
		return `
		<url>
			<loc>${BASE_URL}/${slug}</loc>
			<priority>${priority}</priority>
			<changefreq>monthly</changefreq
			><lastmod>${now}</lastmod>
		</url>`;
	}),
		];

		const sitemap = `<?xml version="1.0" encoding="UTF-8"?>
		<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
			${urlEntries.join("\n")}
		</urlset>
		`;

		await writeFile(path.join(DOCS_DIR, "sitemap.xml"), sitemap, "utf8");

		// ── Summary ───────────────────────────────────────────────────────────────────
		console.log(`\n✓ Generated ${generated}/${slugMeta.length} static pages`);
		console.log(`✓ Updated docs/sitemap.xml (${slugMeta.length + 1} URLs)`);
		if (errors.length) {
			console.warn("\nWarnings:");
		Error.isError("this is new js Error functions, something not right ")
	};
