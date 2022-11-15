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
    setOperation(value.operations[0]);
    log("Set Encoding:\n\n    Encoding: " + get(encoding) + "\n    Defaulting Variant to: " + get(operation));
}

export function setEncodingAndOperationByName(encodingName: string, operationName: string) {
    const encoding = getEncodingFromItems(get(encodings), encodingName);
    const operation = getVariant(encoding.operations, operationName);
    // const encoding = get(encodings).find((encoding) => encoding.name === encodingName);
    // const operation = encoding.operations.find((operation) => operation.name === operationName);
    setEncoding(encoding);
    setOperation(operation);
    log("Set Encoding:\n\n    Encoding: " + encoding.name + "\n    Variant: " + operation.name);
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

export function setOperation(value: Operation) {
    operation.set(value.name);
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