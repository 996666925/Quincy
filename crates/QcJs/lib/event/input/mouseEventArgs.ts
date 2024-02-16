import { Vector3 } from "../../math/vector";
import { EventArgs } from "./eventArgs";
import { MouseButton } from "./index";



export class MouseEventArgs extends EventArgs {
    constructor(public button: MouseButton, public position: Vector3) {
        super()
    }
}