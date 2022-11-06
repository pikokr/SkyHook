#[derive(Debug)]
pub enum VK {
    Escape,

    // Function Keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // 2nd Layer
    Grave,
    Alpha1,
    Alpha2,
    Alpha3,
    Alpha4,
    Alpha5,
    Alpha6,
    Alpha7,
    Alpha8,
    Alpha9,
    Alpha0,
    Minus,
    Equal,
    Backspace,

    // 3rd Layer
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftBrace,
    RightBrace,
    BackSlash,

    // 4th Layer
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Apastrophe,

    // 5th Layer
    LShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    RShift,

    // 6th Layer
    LControl,
    Super,
    LAlt,
    Space,
    RAlt,
    RControl,

    // Controls
    PrintScreen,
    ScrollLock,
    PauseBreak,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    ArrowUp,
    ArrowLeft,
    ArrowDown,
    ArrowRight,

    // Keypad
    NumLock,
    KeypadSlash,
    KeypadAsterisk,
    KeypadMinus,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadDelete,
    KeypadPlus,
    KeypadEnter,

    // Mouse
    MouseLeft,
    MouseRight,
    MouseMiddle,
    MouseX1,
    MouseX2,
}
