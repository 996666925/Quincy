

import { GameObject } from "./gameobject";


function onStart() {
    let comp: Component = this;
    let isDirty = false;
    let proxy = new Proxy(comp, {
        set(target, prop, value, recv) {
            target[prop] = value;
            isDirty = true;
            return true;
        }
    })
    proxy.onStart();
    if (isDirty) {
        return { isDirty, value: JSON.stringify(comp) };
    } else {
        return { isDirty, value: null };
    }
}
function onUpdate(name: string, dt: number) {

    let comp: Component = this;

    let isDirty = false;
    Object.setPrototypeOf(comp, globalThis[`__${name}__`]);
    let proxy = new Proxy(comp, {
        // get(target, prop, recv) {
        //     print((prop as string) + ":" + JSON.stringify(target[prop]))
        //     let value = target[prop];
        //     if (value?.parent != null) {
        //         print("change:")
        //         Reflect.setPrototypeOf(value, globalThis[`__${value.constructor.typeName}__`]);
        //     }
        //     return value
        // },
        set(target, prop, value, recv) {
            target[prop] = value;

            isDirty = true;
            return true;
        }
    })
    proxy.onUpdate(dt);
    if (isDirty) {
        return { isDirty, value: JSON.stringify(comp)  };
    } else {
        return { isDirty, value: null };
    }
}



export class Component {
    static typeName: string = "Component";
    parent: string;
    onStart() { }
    onUpdate(_dt: number) { }
    getComponent<T extends Component>(value: typeof Component): T {
        return Deno.core.ops.op_getComponent(this.parent, value.typeName);
    }

    getGameObject(name: string): GameObject {
        return Deno.core.ops.op_getGameObject(this, name);
    }
}

globalThis.__Component__ = new Component();
globalThis.__ONUPDATE__ = onUpdate;
globalThis.__ONSTART__ = onStart;
// export function Component(name: string) {
//     return (value: any, { kind }: ClassDecoratorContext) => {
//         if (kind == "class") {
//             app.addComponent(name, value);
//         }
//         value.prototype.__proto__ = Comp.prototype;
//     }
// }
