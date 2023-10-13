
import { Component, GameObject, Transform } from "../lib";
import { Example } from "./example";


class Test {
  id = 233
  showId() {
    print("id:" + this.id)
  }
}




class Person extends Component {

  example: Example;
  go: GameObject;
  id = 0;
  onStart() {
    this.example = this.getComponent(Example);
    this.go = this.getGameObject("GameObject")
  }

  onUpdate(dt: number) {
    let rotation = this.go.transform.rotation;
    rotation[1] += dt;
    this.go.transform.setRotation(rotation)

  }
}




let go = GameObject.create("XXX");
go.addComponent(new Person)

let comp = go.getComponent(Person)




