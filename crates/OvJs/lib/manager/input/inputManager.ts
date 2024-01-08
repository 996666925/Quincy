import { VirtualItem, VirtualKeyBoard, VirtualMouse } from './../../event/input/index';
import { Emitter } from "../../event/emitter";
import { InputEvent } from "../../event/input/inputEvent";
import { KeyBoardEventArgs } from '../../event/input/keyboardEventArgs';
import { MouseEventArgs } from '../../event/input/mouseEventArgs';


class InputManager extends Emitter<InputEvent> {

}

export const input = new InputManager();


globalThis.__POST_INPUT_MESSAGE__ = (name: VirtualItem, data: VirtualKeyBoard & VirtualMouse) => {

    (data.state as any) = name + data.state;
    switch (name) {
        case VirtualItem.KEYBOARD:
            input.emit(data.state, new KeyBoardEventArgs(data.virtual_keycode));
            break;
        case VirtualItem.MOUSE:
            input.emit(data.state, new MouseEventArgs(data.button, data.position));
            break;
    }

}