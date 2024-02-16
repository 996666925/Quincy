import { EventArgs } from "./eventArgs";


export class KeyBoardEventArgs extends EventArgs {


    constructor(public key: string) {
        super()
    }
}