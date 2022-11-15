export class Encoding {
    constructor(public name: string, public label: string, public description: string, public operations: Operation[]){}
}

export class Operation {
    constructor(public name: string, public reverse: string, public label: string, public description: string){}
}

export class EncodeOperation {
    constructor(
        public encoding: string,
        public operation: string
    ) {}
}