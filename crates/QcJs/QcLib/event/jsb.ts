import { getComponentById, getGameObjectById } from "../core/gameobject";
import { InputEvent, KeyBoardEventArgs, input } from "../index";
import { VirtualItem, VirtualKeyBoard, VirtualMouse } from "./input/index";
import { MouseEventArgs } from "./input/mouseEventArgs";


enum MessageType {
    KEYBOARD = "keyboard",
    MOUSE = "mouse",
    MOUSE_MOVE = "mouse_move",
    UI = "ui"
}


globalThis.__POST_MESSAGE__ = (type: MessageType, data: any, ...args) => {

    switch (type) {
        case MessageType.KEYBOARD:
            {
                let keyboard = data as VirtualKeyBoard;
                input.emit((type + keyboard.state) as InputEvent, new KeyBoardEventArgs(data.virtual_keycode));
                break;
            }
        case MessageType.MOUSE:
            {
                let mouse = data as VirtualMouse;
                input.emit((type + mouse.state) as InputEvent, new MouseEventArgs(data.button, data.position.toVec()));
                break;
            }
        case MessageType.MOUSE_MOVE:
            {
                let mouse = data as VirtualMouse;
                input.emit((type + mouse.state) as InputEvent, new MouseEventArgs(data.button, data.position.toVec()));
                break;
            }
        case MessageType.UI:
            {
                // print(`type:${type},data:${JSON.stringify(args)}`)
                let uiBind = args[0][0];
                // print(uiBind)
                let comp = getComponentById(uiBind["objId"], uiBind["compId"])
                comp[uiBind["funcName"]]();
                break;
            }
        default:
            print(`type:${type},data:${JSON.stringify(data)}`)
    }

}