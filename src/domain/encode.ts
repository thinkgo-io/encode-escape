import {get, writable} from "svelte/store";
import type {Encoding, Operation} from "../types/encoding";
import {input, result} from "../store/content";
import {encodings, encoding, operation, encodingList, operationList} from "../store/encoding";

import { getEncoding as getEncodingFromItems, getVariant, toEncodingList, toVariantList } from "../domain/encode_utils";
import { log } from "../utils/log";

// Public ─────────────────────────────────────────────── //

export function setEncoding(value: Encoding) {
    encoding.set(value.name);
    operationList.set(toVariantList(value.operations));
    operation.set(value.operations[0].name);
    log("Set Encoding:\n  Encoding: " + get(encoding) + "\n  Variant: " + get(operation));
}

export function setEncodingByName(name: string) {
    const value = get(encodings).find((encoding) => encoding.name === name);
    setEncoding(value);
}

export function setEncodings(values:Encoding[]) {
    encodings.set(values);
    setEncoding(values[0])

    encodingList.set(toEncodingList(get(encodings)));
}

export function swap() {
    swapVariants();
    swapContent();
}

// Private ────────────────────────────────────────────── //

function getEncoding(): Encoding {
    return getEncodingFromItems(get(encodings), get(encoding));
}

function swapContent() {
    const temp = get(input);
    input.set(get(result));
    result.set(temp);
}

function swapVariants() {
    const original = getVariant(getEncoding().operations, get(operation));
    operation.set(original.reverse);
}