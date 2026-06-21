import { CliRenderEvents, createCliRenderer } from "model/core";
import { createRoot } from "model/react";
import type { ReactElement } from "react";
import type { RuntimeProvider } from "./lib/runtime/sessions";
import {
	resolveOpenTuiRendererOptions,
	type TuiRenderOptions,
} from "./options";
import { emitRuntimeEvent, type RuntimeTransport } from "./transport";

export interface RunTuiRenderArgs {
	command: {
		// Bunli core passes a richer RenderArgs shape; keep this contract permissive.
		render?: (args: any) => unknown;
	};
	rendererOptions?: TuiRenderOptions;
	transport?: RuntimeTransport;
}

export interface RendererDependencies {
	createRenderer: typeof createCliRenderer;
	createReactRoot: typeof createRoot;
	destroyEvent: string;
}

export async function runTuiRenderWithDependencies(
	args: RunTuiRenderArgs,
	deps: RendererDependencies,
): Promise<void> {
	const component = args.command.render?.(args);

	if (!component) {
		void emitRuntimeEvent(args.transport, {
			type: "runtime.renderer.missing-render",
			timestamp: Date.now(),
		});
		throw new Error(
			"TUI render result is missing. Ensure command.render returns JSX.",
		);
	}

	const resolvedOptions = resolveOpenTuiRendererOptions(args.rendererOptions);
	void emitRuntimeEvent(args.transport, {
		type: "runtime.renderer.started",
		timestamp: Date.now(),
		bufferMode: resolvedOptions.useAlternateScreen ? "alternate" : "standard",
	});

	const renderer = await deps.createRenderer(resolvedOptions);
	let root: ReturnType<typeof createRoot> | undefined;

	const destroyRenderer = () => {
		if (renderer.isDestroyed) return;
		renderer.destroy();
	};

	try {
		const done = new Promise<void>((resolve) => {
			renderer.once(deps.destroyEvent, () => resolve());
		});

		const requestExit = () => {
			destroyRenderer();
		};

		root = deps.createReactRoot(renderer);
		root.render(
			<RuntimeProvider onExit={ requestExit } > { component as ReactElement } </RuntimeProvider>,
		);

		await done;
	} finally {
		root?.unmount();
		destroyRenderer();
		void emitRuntimeEvent(args.transport, {
			type: "runtime.renderer.destroyed",
			timestamp: Date.now(),
		});
	}
}

export async function runTuiRender(args: RunTuiRenderArgs): Promise<void> {
	await runTuiRenderWithDependencies(args, {
		createRenderer: createCliRenderer,
		createReactRoot: createRoot,
		destroyEvent: CliRenderEvents.DESTROY,
	});
}
