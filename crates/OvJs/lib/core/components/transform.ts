import { Component } from "../component";


class Rotation {
    x: number;
    y: number;
    z: number;
}


export class Transform extends Component {

    position: Array<number>;
    rotation: Rotation;
    setPosition(position: Array<number>) {
        return Deno.core.ops.opSetPosition(this, position);
    }
    setRotation(rotation: Array<number>) {
        return Deno.core.ops.opSetRotation(this, rotation);
    }

}

globalThis.__Transform__ = new Transform();
