import { invoke } from "@tauri-apps/api/tauri";

import type { Encoding } from "../types/encode";

export async function log(message: String) {
    return invoke("on_log" , {
        message: message,
    });
}

export async function logError(context: string, error: Error) {
    return invoke("on_log_error", {
        context: context,
        message: error.message + "\n\n" + error.stack,
      });
}