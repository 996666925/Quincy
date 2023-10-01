

import { GameObject } from "./gameobject";

function onUpdate(obj: string, name: string, dt: number) {
    let comp = JSON.parse(obj);
    let isDirty = false;
    Object.setPrototypeOf(comp, globalThis[`__${name}__`]);
    let proxy = new Proxy(comp, {
        set(target, prop, value, recv) {
            target[prop] = value;
            isDirty = true;
            return true;
        }
    })
    proxy.onUpdate(dt);
    if (isDirty) {
        return { isDirty, value: JSON.stringify(comp) };
    } else {
        return { isDirty };
    }
}

export class Component {
    static typeName: string;
    onStart() { }
    onUpdate(_dt: number) { }
    getComponent(value: typeof Component) {
        Deno.core.ops.op_getComponent(this, value.typeName);
    }

    getGameObject(name: string): GameObject {
        return Deno.core.ops.op_getGameObject(this, name);
    }
}

globalThis.__COMPONENT__ = new Component();
globalThis.__ONUPDATE__ = onUpdate;
// export function Component(name: string) {
//     return (value: any, { kind }: ClassDecoratorContext) => {
//         if (kind == "class") {
//             app.addComponent(name, value);
//         }
//         value.prototype.__proto__ = Comp.prototype;
//     }
// }
