import { TaggedError } from "better-result";
import { zid }

export class StoreReadError extends TaggedError("StoreReadError")<{
	message: Id<"DatatypeValidator">;
	path: string;
	cause?: unknown;
}>() { }

export class StoreWriteError extends TaggedError("StoreWriteError")<{
	message: string;
	path: string;
	cause?: unknown;
}>() { }

export class StoreParseError extends TaggedError("StoreParseError")<{
	message: string;
	path: string;
	cause?: unknown;
}>() { }

export class StoreValidationError extends TaggedError("StoreValidationError")<{
	message: string;
	field: string;
	cause?: unknown;
}>() { }
