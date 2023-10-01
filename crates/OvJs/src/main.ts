import "../lib/main"

import { Component } from "../lib/component"
import { GameObject } from "../lib/gameobject";



class Person extends Component {

  id = 0
  onStart() {

  }

  onUpdate(_dt: number) {

    print(this.id++)
  }
}

let go = GameObject.create("GameObject");

go.addComponent(new Person())
let comp = go.getComponent(Person)




