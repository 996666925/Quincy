/// <reference path="../QcLib/vite-env.d.ts" />

import { Component, GameObject, Transform, input, KeyBoardEventArgs, InputEvent, Keys, getGameObject } from "../QcLib";
import { MouseEventArgs } from "../QcLib/event/input/mouseEventArgs";
import { Vector3 } from "../QcLib/math/vector";

import { Example } from "./example";
import clamp from "lodash/clamp"




class Cube extends Component {
  //旋转角度
  private  xRotation = 0.0;
  private  yRotation = 0.0;
  preX=0;
  preY=0;

  speed = 0.1;
  camera:GameObject
  onStart() {
    this.camera=this.getGameObject("Camera");
    // input.on(InputEvent.KEY_DOWN, this.onKeyDown, this);
    // input.on(InputEvent.MOUSE_MOVE, this.onMouseMove, this);
  }

  onMouseMove(args: MouseEventArgs) {

    if(this.preX==0&&this.preY==0){
      this.preX=args.position.x;
      this.preY=args.position.y;
      return;
    }
    
    let diffX=args.position.x-this.preX;
    let diffY=args.position.y-this.preY;

     this.yRotation -= diffX;
     this.xRotation -= diffY;

     this.xRotation =clamp(this.xRotation,-60,60);

     this.camera.transform.setRotation(new Vector3(this.xRotation,this.yRotation,0))


     this.preX=args.position.x;
     this.preY=args.position.y;
  }


  onKeyDown(args: KeyBoardEventArgs) {

    print(`你按下了${args.key}键`)
    if (args.key == Keys.D) {
      this.camera.transform.translate(new Vector3(this.speed, 0, 0))
    } else if (args.key == Keys.A) {
      this.camera.transform.translate(new Vector3(-this.speed, 0, 0))
    }

    if (args.key == Keys.W) {
      this.camera.transform.translate(new Vector3(0, 0, -this.speed))
    } else if (args.key == Keys.S) {
      this.camera.transform.translate(new Vector3(0, 0, this.speed))
    }

  }

  onUpdate(dt: number) {
   
    this.speed += dt;
    this.node.transform.setRotation(new Vector3(0,this.speed,0))
  }


  onClick(){
    print("按钮被点击了！")
  }
}


// class Cube extends Component {
//   speed = 0.016;
//   onStart(): void {
      
//   }
//   onUpdate(dt: number) {
//     //获取物体的坐标
//     let position = this.node.transform.position;
//     if (position.x > 3) {
//       this.speed = -this.speed;
//     } else if (position.x < -3) {
//       this.speed = -this.speed;
//     } 
//     //移动物体
//     this.node.transform.translate(new Vector3(this.speed, 0, 0))
//   }
// }




