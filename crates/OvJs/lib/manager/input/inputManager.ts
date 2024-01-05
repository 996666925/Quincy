import { VirtualKeyBoard } from './../../event/input/index';
import { Emitter } from "../../event/emitter";
import { InputEvent } from "../../event/input/inputEvent";
import { InputEventArgs } from '../../event/input/inputEventArgs';


class InputManager extends Emitter<InputEvent> {

}

export const input = new InputManager();

globalThis.__POST_INPUT_MESSAGE__ = (data: VirtualKeyBoard) => {

    input.emit(data.state, new InputEventArgs(data.virtual_keycode));
}