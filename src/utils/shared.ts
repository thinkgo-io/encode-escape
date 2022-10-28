// Core ───────────────────────────────────────────────────────────────────── //

export const fromId            = (value: string) => document.getElementById(value);
export const isEmpty           = (value: string) => isNullOrUndefined(value) || value === "";
export const isDefined         = value => typeof(value) !== "undefined";
export const isNull            = value => value === null;
export const isNullOrUndefined = value => isNull(value) || isUndefined(value);
export const isUndefined       = value => !isDefined(value);
export const notEmpty          = (value: string) => !isEmpty(value);
export const notDefined        = value => !isDefined(value);
export const notNull           = value => !isNull(value);

export const fireAndForget = (func: Function) => setTimeout(func, 0);
