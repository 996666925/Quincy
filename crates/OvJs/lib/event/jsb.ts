import { InputEvent, KeyBoardEventArgs, input } from "../index";
import { VirtualItem, VirtualKeyBoard, VirtualMouse } from "./input/index";
import { MouseEventArgs } from "./input/mouseEventArgs";


enum MessageType {
    KEYBOARD = "keyboard",
    MOUSE = "mouse",
    MOUSE_MOVE = "mouse_move"
}


globalThis.__POST_MESSAGE__ = (type: MessageType, data: any) => {

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
    }

}