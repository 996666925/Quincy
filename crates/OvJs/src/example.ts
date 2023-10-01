import { Component } from "../lib/component"

export class Example extends Component {
    id = 233
    onUpdate(dt: number) {
        print("id:" + this.id)
    }
}