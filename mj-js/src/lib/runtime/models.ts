export const TUI_CANCEL = Symbol.for("bun:prompt_cancel");

export type TuiCancel = typeof TUI_CANCEL;

export type PromptResolver<T> = (value: T | TuiCancel) => void;

export interface OpenTuiSelectOption<T = string> {
	label: string;
	value: T;
	hint?: string;
	disabled?: boolean;
}
