import { invoke } from "@tauri-apps/api/tauri";

import type { Encoding } from "../types/encoding";

export async function getEncodings(): Promise<Encoding[]> {
    return invoke("on_get_encodings");
}

export async function encode(encoding: string, operation: string, input: string): Promise<string> {
    return invoke("on_encode", {
        encoding: encoding,
        operation: operation,
        input: input,
      });
}