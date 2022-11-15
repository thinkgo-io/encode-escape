import { invoke } from "@tauri-apps/api/tauri";

import type { Encoding } from "../types/encoding";
import { EncodeOperation } from "../types/encoding";
import { log } from "../utils/log";


export async function getCurrentEncodeOperation(): Promise<EncodeOperation> {

    try {
        return await invoke("on_get_current_encode_operation");
    } catch (error) {
        log ("UI - getCurrentEncodingOperation: Error: " + error);
        return Promise.resolve().then((_) => { return new EncodeOperation("base64", "encode"); });
    }
}

export async function getEncodings(): Promise<Encoding[]> {
    return invoke("on_get_encodings")
}

export async function encode(encodeOperation: EncodeOperation, input: string): Promise<string> {

    try {
        return await invoke("on_encode", {
            encodeOperation: encodeOperation,
            input: input,
        });
    } catch (error) {
        return "[Not Encodable]";
    }
}

export async function encode_1(encodeOperation: EncodeOperation, input: string): Promise<string> {
    return invoke("on_encode", {
        encodeOperation: encodeOperation,
        input: input,
      });
}