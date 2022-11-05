import { isUndefined, fireAndForget } from "./shared";
import { log as send_log, logError as send_log_error } from "../backend/log";

export const TO_BACKEND = false;
export const SKIP_BACKEND = true;

// Public ─────────────────────────────────────────────── //

export function log(message: string, toBackend = SKIP_BACKEND) {

	console.log(message);
	// if (toBackend)
	// 	fireAndForget(() => send_log(message));
}

export function logError(context: string, error: Error) {

	console.error(context, error);
  	console.trace();
	fireAndForget(() => send_log_error(context, error));
	alert("Error: " + context + "\n\n" + error.message);
}