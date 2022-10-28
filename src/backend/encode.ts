import { invoke } from "@tauri-apps/api/tauri";

import type { Encoding } from "../types/encode";

export async function getEncodings(): Promise<Encoding[]> {
    return invoke("get_encodings");
}

export async function encode(encoding: string, variant: string, input: string): Promise<string> {
    return invoke("on_encode", {
        encoding: encoding,
        variant: variant,
        input: input,
      });
}