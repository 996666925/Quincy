import { Transform } from "../index";
import { Component } from "./component"



//js创建go时,在rust处也创建一个go,rust存储全部的go,js保持go的映射,通过name
export class GameObject {

    _transform: Transform;

    get transform() {

        let handler = {
            get(target, prop) {
                let result = Reflect.get(target, prop)
                if (typeof result == "object") {
                    result.__prop__ = prop;
                    result.__target__ = target;
                    return new Proxy(result, handler)
                }

                return result;

            },
            set(target, prop, newValue) {
                let key = target.__prop__ ;
                let transform: Transform = target.__target__;
                Reflect.set(target, prop, newValue)

                switch (key) {
                    case 'rotation':
                        transform.setRotation(target)
                        break;
                    case 'position':
                        transform.setPosition(target)
                        break;
                    default:
                        break;
                }


                return true;
            }
        };
        return new Proxy(this._transform, handler)
    }

    //不能使用自带的构造方法
    constructor(private name: string) { }


    static create(name: string): GameObject {
        return Deno.core.ops.op_createGameObject(name);
    }


    addComponent(value: Component) {
        let type: any = value.constructor;
        Deno.core.ops.op_addComponent(this.name, value, type.typeName);
    }

    getComponent<T extends Component>(value: typeof Component): T {
        return Deno.core.ops.op_getComponent(this.name, value.typeName);
    }

}


export function getGameObject(name: string): GameObject {
    return Deno.core.ops.op_getGameObject(this, name);
}
globalThis.__GAMEOBJECT__ = new GameObject("");