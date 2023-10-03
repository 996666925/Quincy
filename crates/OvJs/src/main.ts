
import { Component, GameObject } from "../lib";
import { Example } from "./example";


class Test {
  id = 233
  showId() {
    print("id:" + this.id)
  }
}


class Person extends Component {

  example: Example;
  id = 0;
  onStart() {
    this.example = this.getComponent(Example);
    // print("onStart" + this.example)
  }

  onUpdate(_dt: number) {

    this.example.id++;
  }
}

let go = GameObject.create("GameObject");
go.addComponent(new Example)
go.addComponent(new Person)

let comp = go.getComponent(Person)




