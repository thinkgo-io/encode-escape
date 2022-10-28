import {writable} from "svelte/store";
import type {Writable} from "svelte/store";

export const input:Writable<string> = writable("");
export const result:Writable<string> = writable("");