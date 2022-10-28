export class Encoding {
    constructor(public name: string, public label: string, public description: string, public variants: Variant[]){}
}

export class Variant {
    constructor(public name: string, public reverse: string, public label: string, public description: string){}
}