
import { Component, GameObject, Transform, input, InputEventArgs, InputEvent, Keys, getGameObject } from "../lib";

import { Example } from "./example";





class Cube extends Component {


  speed = 0.1;
  onStart() {
    input.on(InputEvent.KEY_DOWN, this.onKeyDown, this);
  }

  onKeyDown(args: InputEventArgs) {

    print(`你按下了${args.key}键`)
    let rotation = this.node.transform.rotation;
    if (args.key == Keys.D) {

      rotation.y += this.speed;
    } else if (args.key == Keys.A) {

      rotation.y -= this.speed;
    }
  }

  onUpdate(dt: number) {

  }
}

// let go = GameObject.create("XXX");

let go = getGameObject( "GameObject")
go.addComponent(new Cube)

// let comp = go.getComponent(Cube)




