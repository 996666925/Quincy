

import { Transform } from "../index";
import { GameObject } from './gameobject';




export class Component {
    static typeName: string = "Component";
    name: string;
    parent: string;
    transform: Transform

    _node: GameObject;
    get node() {
        if (this._node == null) {
            this._node = this.getGameObject(this.parent);
        }
        return this._node;
    }
    onStart() { }
    onUpdate(dt: number) {

    }
    getComponent<T extends Component>(value: typeof Component): T {
        return Deno.core.ops.op_getComponent(this.parent, value.typeName);
    }

    getGameObject(name: string): GameObject {
        return Deno.core.ops.op_getGameObject(this, name);
    }
}

globalThis.__Component__ = new Component();

