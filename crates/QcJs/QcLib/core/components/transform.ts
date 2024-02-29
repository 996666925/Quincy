
import { Vector3 } from '../../math/vector';
import { Component } from "../component";




export class Transform extends Component {


    get position(): Vector3 {

        return Deno.core.ops.opGetPosition(this).toVec();
    }

    setPosition(position: Vector3) {
        return Deno.core.ops.opSetPosition(this, position.into());
    }

    get rotation(): Vector3 {
        return Deno.core.ops.opGetRotation(this).toVec();
    }

    setRotation(rotation: Vector3) {
        return Deno.core.ops.opSetRotation(this, rotation.into());
    }

    translate(vector: Vector3) {
        return Deno.core.ops.opTranslate(this, vector.into());
    }


}

globalThis.__Transform__ = new Transform();
