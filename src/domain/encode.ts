import {get, writable} from "svelte/store";
import type {Encoding, Variant} from "../types/encode";
import {input, result} from "../store/content";
import {encodings, encoding, variant, encodingList, variantList} from "../store/encode";

import { getEncoding as getEncodingFromItems, getVariant, toEncodingList, toVariantList } from "../domain/encode_utils";
import { log } from "../utils/log";

// Public ─────────────────────────────────────────────── //

export function setEncoding(value: Encoding) {
    encoding.set(value.name);
    variantList.set(toVariantList(value.variants));
    variant.set(value.variants[0].name);
    log("Set Encoding:\n  Encoding: " + get(encoding) + "\n  Variant: " + get(variant));
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
    const original = getVariant(getEncoding().variants, get(variant));
    variant.set(original.reverse);
}