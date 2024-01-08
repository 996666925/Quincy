
import { Component, GameObject, Transform, input, KeyBoardEventArgs, InputEvent, Keys, getGameObject } from "../lib";
import { MouseEventArgs } from "../lib/event/input/mouseEventArgs";
import { Vector3 } from "../lib/math/vector";

import { Example } from "./example";





class Cube extends Component {


  speed = 0.1;
  onStart() {

    // input.on(InputEvent.KEY_DOWN, this.onKeyDown, this);
    // input.on(InputEvent.MOUSE_UP, this.onMouseDown, this);
  }

  onMouseDown(args: MouseEventArgs) {
    print(args)
  }

  onKeyDown(args: KeyBoardEventArgs) {

    print(`你按下了${args.key}键`)

    if (args.key == Keys.D) {
      this.node.transform.translate(new Vector3(this.speed, 0, 0))
    } else if (args.key == Keys.A) {

      this.node.transform.translate(new Vector3(-this.speed, 0, 0))
    }
    print(this.node.transform.position)
  }

  onUpdate(dt: number) {
    let position = this.node.transform.position;
    if (position.x > 3) {
      this.speed = -dt * 4;
    } else if (position.x < -3) {
      this.speed = dt * 4;
    } else {

    }
    this.node.transform.translate(new Vector3(this.speed, 0, 0))
  }
}

// let go = GameObject.create("XXX");

// let go = getGameObject( "GameObject")
// go.addComponent(new Cube)

// let comp = go.getComponent(Cube)




