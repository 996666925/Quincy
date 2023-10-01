import { Component } from "./component"


//js创建go时,在rust处也创建一个go,rust存储全部的go,js保持go的映射,通过name
export class GameObject {



    //不能使用自带的构造方法
    constructor(private name: string) { }


    static create(name: string): GameObject {
        return Deno.core.ops.op_createGameObject(name);
    }


    addComponent(value: Component) {
        let type: any = value.constructor;
        Deno.core.ops.op_addComponent(this.name, JSON.stringify(value), type.typeName);
    }
    getComponent(comp: typeof Component): Component {
        return Deno.core.ops.op_getComponent(this.name, comp.typeName);
    }

}

globalThis.__GAMEOBJECT__ = new GameObject("");