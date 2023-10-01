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

  example = new Example();
  test = new Test();
  id = 0
  onStart() {

  }

  onUpdate(_dt: number) {
  }
}

let go = GameObject.create("GameObject");

go.addComponent(new Person())
let comp = go.getComponent(Person)




