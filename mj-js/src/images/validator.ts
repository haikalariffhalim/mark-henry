import type { ImageRenderMode } from "./model";

export interface ResolveImageRenderValidator {
	flagMode?: ImageRenderMode;
	configMode?: ImageRenderMode;
	defaultMode?: ImageRenderMode;
}

export function resolveImageRenderMode(input: ResolveImageRenderValidator = {}): ImageRenderMode {
	if (input.flagMode) return input.flagMode;
	if (input.configMode) return input.configMode;
	return input.defaultMode ?? "auto";
}

export function shouldFailOnImageMiss(mode: ImageRenderMode): boolean {
	return mode === "on";
}
