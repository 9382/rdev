use crate::rdev::Key;
use std::convert::TryInto;
use winapi::shared::minwindef::WORD;

macro_rules! decl_keycodes {
    ($($key:ident, $code:literal, $scan:literal),*) => {
        //TODO: make const when rust lang issue #49146 is fixed
        pub fn virtcode_from_key(key: Key) -> Option<WORD> {
            match key {
                $(
                    Key::$key => Some($code),
                )*
                Key::Unknown(code) => Some(code.try_into().ok()?),
                _ => None,
            }
        }

        pub fn scancode_from_key(key: Key) -> Option<WORD> {
            match key {
                $(
                    Key::$key => Some($scan),
                )*
                Key::Unknown(_) => None,
                _ => None,
            }
        }

        //TODO: make const when rust lang issue #49146 is fixed
        pub fn key_from_virtcode(code: WORD) -> Key {
            match code {
                $(
                    $code => Key::$key,
                )*
                _ => Key::Unknown(code.into())
            }
        }
    };
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes for the virtual key codes
// https://aeb.win.tue.nl/linux/kbd/scancodes-10.html#ss10.6 for the hardware scan codes
// We redefined here for Letter and number keys which are not in winapi crate (and don't have a name either in win32)
decl_keycodes! {
    Alt, 164, 0x38,
    AltGr, 165, 541,
    Backspace, 0x08, 0x0e,
    CapsLock, 20, 0x3a,
    ControlLeft, 162, 0x1d,
    ControlRight, 163, 0xe01d,
    Delete, 46, 0xe053,
    DownArrow, 40, 0xe050,
    End, 35, 0xe04f,
    Escape, 27, 0x01,
    F1, 112, 0x3b,
    F10, 121, 0x44,
    F11, 122, 0x45,
    F12, 123, 0x46,
    F2, 113, 0x3c,
    F3, 114, 0x3d,
    F4, 115, 0x3e,
    F5, 116, 0x3f,
    F6, 117, 0x40,
    F7, 118, 0x41,
    F8, 119, 0x42,
    F9, 120, 0x43,
    Home, 36, 0xe047,
    LeftArrow, 37, 0xe04b,
    MetaLeft, 91, 0xe05b,
    MetaRight, 92, 0xe05c,
    PageDown, 34, 0xe051,
    PageUp, 33, 0xe049,
    Return, 0x0D, 0x1c,
    RightArrow, 39, 0xe04d,
    ShiftLeft, 160, 0x2a,
    ShiftRight, 161, 0x36,
    Space, 32, 0x39,
    Tab, 0x09, 0x0f,
    UpArrow, 38, 0xe048,
    PrintScreen, 44, 0xe037,
    ScrollLock, 145, 0x46,
    Pause, 19, 0, // No simple translation (3 bytes)
    NumLock, 144, 0x45,
    BackQuote, 192, 0x29,
    Num1, 49, 0x02,
    Num2, 50, 0x03,
    Num3, 51, 0x04,
    Num4, 52, 0x05,
    Num5, 53, 0x06,
    Num6, 54, 0x07,
    Num7, 55, 0x08,
    Num8, 56, 0x09,
    Num9, 57, 0x0a,
    Num0, 48, 0x0b,
    Minus, 189, 0x0c,
    Equal, 187, 0x0d,
    KeyQ, 81, 0x10,
    KeyW, 87, 0x11,
    KeyE, 69, 0x12,
    KeyR, 82, 0x13,
    KeyT, 84, 0x14,
    KeyY, 89, 0x15,
    KeyU, 85, 0x16,
    KeyI, 73, 0x17,
    KeyO, 79, 0x18,
    KeyP, 80, 0x19,
    LeftBracket, 219, 0x1a,
    RightBracket, 221, 0x1b,
    KeyA, 65, 0x1e,
    KeyS, 83, 0x1f,
    KeyD, 68, 0x20,
    KeyF, 70, 0x21,
    KeyG, 71, 0x22,
    KeyH, 72, 0x23,
    KeyJ, 74, 0x24,
    KeyK, 75, 0x25,
    KeyL, 76, 0x26,
    SemiColon, 186, 0x27,
    Quote, 222, 0x28,
    BackSlash, 220, 0x2b,
    IntlBackslash, 226, 0x2b, // Not sure for the hw scancode here
    KeyZ, 90, 0x2c,
    KeyX, 88, 0x2d,
    KeyC, 67, 0x2e,
    KeyV, 86, 0x2f,
    KeyB, 66, 0x30,
    KeyN, 78, 0x31,
    KeyM, 77, 0x32,
    Comma, 188, 0x33,
    Dot, 190, 0x34,
    Slash, 191, 0x35,
    Insert, 45, 0xe052,
    //KP_RETURN, 13, 0xe01c,
    KpMinus, 109, 0x4a,
    KpPlus, 107, 0x4e,
    KpMultiply, 106, 0x37,
    KpDivide, 111, 0xe035,
    Kp0, 96, 0x52,
    Kp1, 97, 0x4f,
    Kp2, 98, 0x50,
    Kp3, 99, 0x51,
    Kp4, 100, 0x4b,
    Kp5, 101, 0x4c,
    Kp6, 102, 0x4d,
    Kp7, 103, 0x47,
    Kp8, 104, 0x48,
    Kp9, 105, 0x49,
    KpDelete, 110, 0x53
}

#[cfg(test)]
mod test {
    use super::{virtcode_from_key, key_from_virtcode};
    #[test]
    fn test_reversible() {
        for code in 0..65535 {
            let key = key_from_virtcode(code);
            if let Some(code2) = virtcode_from_key(key) {
                assert_eq!(code, code2)
            } else {
                assert!(false, "We could not convert back code: {:?}", code);
            }
        }
    }
}
