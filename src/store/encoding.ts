import {get, writable} from "svelte/store";
import type {Writable} from "svelte/store";
import type {Encoding} from "../types/encoding";
import type {ListItem} from "../types/listItem";

export const encodings:Writable<Encoding[]> = writable([]);
export const encoding:Writable<string> = writable("");      // Stored as strings to make it easy for the dropdown to set the value.
export const operation:Writable<string> = writable("");       // Stored as strings to make it easy for the radio button to set the value.

export const encodingList: Writable<ListItem[]> = writable([]);
export const operationList: Writable<ListItem[]> = writable([]);