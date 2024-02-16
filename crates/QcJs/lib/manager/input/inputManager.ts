import { VirtualItem, VirtualKeyBoard, VirtualMouse } from './../../event/input/index';
import { Emitter } from "../../event/emitter";
import { InputEvent } from "../../event/input/inputEvent";
import { KeyBoardEventArgs } from '../../event/input/keyboardEventArgs';
import { MouseEventArgs } from '../../event/input/mouseEventArgs';


class InputManager extends Emitter<InputEvent> {

}

export const input = new InputManager();
