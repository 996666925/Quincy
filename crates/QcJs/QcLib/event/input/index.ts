import { InputEvent } from "./inputEvent";

export enum VirtualItem {
    KEYBOARD = "keyboard",
    MOUSE = "mouse"
}
export class VirtualKeyBoard {

    scancode: number;
    state: InputEvent;

    key: string;

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
    Backquote = "Backquote",




    Backslash = "Backslash",

    BracketLeft = "BracketLeft",

    BracketRight = "BracketRight",

    Comma = "Comma",

    Digit0 = "Digit0",

    Digit1 = "Digit1",

    Digit2 = "Digit2",

    Digit3 = "Digit3",

    Digit4 = "Digit4",

    Digit5 = "Digit5",

    Digit6 = "Digit6",

    Digit7 = "Digit7",

    Digit8 = "Digit8",

    Digit9 = "Digit9",

    Equal = "Equal",


    IntlBackslash = "IntlBackslash",


    IntlRo = "IntlRo",



    IntlYen = "IntlYen",


    KeyA = "KeyA",

    KeyB = "KeyB",

    KeyC = "KeyC",

    KeyD = "KeyD",

    KeyE = "KeyE",

    KeyF = "KeyF",

    KeyG = "KeyG",

    KeyH = "KeyH",

    KeyI = "KeyI",

    KeyJ = "KeyJ",

    KeyK = "KeyK",

    KeyL = "KeyL",

    KeyM = "KeyM",

    KeyN = "KeyN",

    KeyO = "KeyO",

    KeyP = "KeyP",


    KeyQ = "KeyQ",

    KeyR = "KeyR",

    KeyS = "KeyS",

    KeyT = "KeyT",

    KeyU = "KeyU",

    KeyV = "KeyV",


    KeyW = "KeyW",

    KeyX = "KeyX",


    KeyY = "KeyY",



    KeyZ = "KeyZ",

    Minus = "Minus",

    Period = "Period",

    Quote = "Quote",

    Semicolon = "Semicolon",

    Slash = "Slash",

    AltLeft = "AltLeft",


    AltRight = "AltRight",


    Backspace = "Backspace",

    CapsLock = "CapsLock",


    ContextMenu = "ContextMenu",

    ControlLeft = "ControlLeft",

    ControlRight = "ControlRight",

    Enter = "Enter",

    SuperLeft = "SuperLeft",

    SuperRight = "SuperRight",

    ShiftLeft = "ShiftLeft",

    ShiftRight = "ShiftRight",

    Space = "Space",

    Tab = "Tab",

    Convert = "Convert",

    KanaMode = "KanaMode",



    Lang1 = "Lang1",



    Lang2 = "Lang2",

    Lang3 = "Lang3",

    Lang4 = "Lang4",

    Lang5 = "Lang5",

    NonConvert = "NonConvert",





    Delete = "Delete",

    End = "End",

    Help = "Help",

    Home = "Home",

    Insert = "Insert",

    PageDown = "PageDown",

    PageUp = "PageUp",

    ArrowDown = "ArrowDown",

    ArrowLeft = "ArrowLeft",

    ArrowRight = "ArrowRight",

    ArrowUp = "ArrowUp",

    NumLock = "NumLock",

    Numpad0 = "Numpad0",

    Numpad1 = "Numpad1",

    Numpad2 = "Numpad2",

    Numpad3 = "Numpad3",

    Numpad4 = "Numpad4",

    Numpad5 = "Numpad5",

    Numpad6 = "Numpad6",


    Numpad7 = "Numpad7",

    Numpad8 = "Numpad8",


    Numpad9 = "Numpad9",

    NumpadAdd = "NumpadAdd",

    NumpadBackspace = "NumpadBackspace",





    NumpadClear = "NumpadClear",

    NumpadClearEntry = "NumpadClearEntry",


    NumpadComma = "NumpadComma",


    NumpadDecimal = "NumpadDecimal",

    NumpadDivide = "NumpadDivide",
    NumpadEnter = "NumpadEnter",

    NumpadEqual = "NumpadEqual",


    NumpadHash = "NumpadHash",

    NumpadMemoryAdd = "NumpadMemoryAdd",

    NumpadMemoryClear = "NumpadMemoryClear",

    NumpadMemoryRecall = "NumpadMemoryRecall",

    NumpadMemoryStore = "NumpadMemoryStore",

    NumpadMemorySubtract = "NumpadMemorySubtract",




    NumpadMultiply = "NumpadMultiply",

    NumpadParenLeft = "NumpadParenLeft",

    NumpadParenRight = "NumpadParenRight",







    NumpadStar = "NumpadStar",

    NumpadSubtract = "NumpadSubtract",

    Escape = "Escape",

    Fn = "Fn",


    FnLock = "FnLock",

    PrintScreen = "PrintScreen",

    ScrollLock = "ScrollLock",

    Pause = "Pause",



    BrowserBack = "BrowserBack",
    BrowserFavorites = "BrowserFavorites",

    BrowserForward = "BrowserForward",

    BrowserHome = "BrowserHome",
    BrowserRefresh = "BrowserRefresh",
    BrowserSearch = "BrowserSearch",
    BrowserStop = "BrowserStop",


    Eject = "Eject",

    LaunchApp1 = "LaunchApp1",

    LaunchApp2 = "LaunchApp2",
    LaunchMail = "LaunchMail",
    MediaPlayPause = "MediaPlayPause",
    MediaSelect = "MediaSelect",
    MediaStop = "MediaStop",
    MediaTrackNext = "MediaTrackNext",
    MediaTrackPrevious = "MediaTrackPrevious",


    Power = "Power",
    Sleep = "Sleep",
    AudioVolumeDown = "AudioVolumeDown",
    AudioVolumeMute = "AudioVolumeMute",
    AudioVolumeUp = "AudioVolumeUp",
    WakeUp = "WakeUp",
    Meta = "Meta",
    Hyper = "Hyper",
    Turbo = "Turbo",
    Abort = "Abort",
    Resume = "Resume",
    Suspend = "Suspend",

    Again = "Again",

    Copy = "Copy",

    Cut = "Cut",

    Find = "Find",

    Open = "Open",

    Paste = "Paste",

    Props = "Props",

    Select = "Select",

    Undo = "Undo",

    Hiragana = "Hiragana",

    Katakana = "Katakana",


    F1 = "F1",


    F2 = "F2",


    F3 = "F3",


    F4 = "F4",


    F5 = "F5",


    F6 = "F6",


    F7 = "F7",


    F8 = "F8",


    F9 = "F9",


    F10 = "F10",


    F11 = "F11",


    F12 = "F12",


    F13 = "F13",


    F14 = "F14",


    F15 = "F15",


    F16 = "F16",


    F17 = "F17",


    F18 = "F18",


    F19 = "F19",


    F20 = "F20",


    F21 = "F21",


    F22 = "F22",


    F23 = "F23",


    F24 = "F24",

    F25 = "F25",

    F26 = "F26",

    F27 = "F27",

    F28 = "F28",

    F29 = "F29",

    F30 = "F30",

    F31 = "F31",

    F32 = "F32",

    F33 = "F33",

    F34 = "F34",

    F35 = "F35",

}