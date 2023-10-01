import "../lib/main"
import "./example"
import { Component } from "../lib/component"
import { GameObject } from "../lib/gameobject";
import { Example } from "./example";


class Test {
  id = 233
  showId() {
    print("id:" + this.id)
  }
}


class Person extends Component {

  example: Example;
  id = 0
  onStart() {
    print("Person onStart")
    this.example = this.getComponent(Example);
  }

  onUpdate(_dt: number) {
    print("成功获取到Example:" + this.example.name)
  }
}

let go = GameObject.create("GameObject");
go.addComponent(new Example())
go.addComponent(new Person())

let comp = go.getComponent(Person)




