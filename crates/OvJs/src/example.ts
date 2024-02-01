import { Component } from "../lib";

export class Example extends Component {
    id = 0
    name = "我是Example组件"

    onStart(): void {
        
    }

    onUpdate(dt: number) {
        print("example:" + this.id)

    }


}

