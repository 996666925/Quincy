import { InputEvent } from "./inputEvent";

export enum VirtualItem {
    KEYBOARD = "keyboard",
    MOUSE = "mouse"
}
export class VirtualKeyBoard {
    
    scancode: number;
    state: InputEvent ;

    virtual_keycode: string;

    modifiers: { shift: boolean, ctrl: boolean, alt: boolean }
}

export class VirtualMouse {

    state: InputEvent;

    button: MouseButton;

    position: []
}

export enum MouseButton {
    Left = "Left",
    Right = "Right",
    Middle = "Middle",
}
export enum Keys {
    A = "A",
    B = "B",
    C = "C",
    D = "D",
    E = "E",
    F = "F",
    G = "G",
    H = "H",
    I = "I",
    J = "J",
    K = "K",
    L = "L",
    M = "M",
    N = "N",
    O = "O",
    P = "P",
    Q = "Q",
    R = "R",
    S = "S",
    T = "T",
    U = "U",
    V = "V",
    W = "W",
    X = "X",
    Y = "Y",
    Z = "Z"

}