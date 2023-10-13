import { Component } from "../component";

export class Transform extends Component {

    position: Array<number>;
    rotation: Array<number>
    setPosition(position: Array<number>) {
        return Deno.core.ops.opSetPosition(this, position);
    }
    setRotation(rotation: Array<number>) {
        return Deno.core.ops.opSetRotation(this, rotation);
    }

}

globalThis.__Transform__ = new Transform();
