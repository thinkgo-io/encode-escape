import type { Encoding, Operation } from '../types/encoding';
import { ListItem } from "../types/listItem";

// Public ─────────────────────────────────────────────── //

export function getEncoding(encodings: Encoding[], name: string): Encoding {
    return encodings.find(encoding => encoding.name === name);
}

export function getVariant(variants: Operation[], name: string): Operation {
    return variants.find(variant => variant.name === name);
}

export function toEncodingList(items: Encoding[]): ListItem[] {
    return items.map(toEncodingItem);
}

export function toVariantList(items: Operation[]): ListItem[] {
    return items.map(toVariantItem);
}

// Private ────────────────────────────────────────────── //

function toEncodingItem(encoding: Encoding): ListItem {
    return new ListItem(encoding.name, encoding.label, encoding.description);
}
  
function toVariantItem(variant: Operation): ListItem {
    return new ListItem(variant.name, variant.label, variant.description);
}