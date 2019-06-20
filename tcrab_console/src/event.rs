#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Escape,

    Up,
    Down,
    Left,
    Right,

    #[doc(hidden)]
    _NonExhaustive,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Clone)]
pub enum Event {
    Quit,
    KeyboardInput {
        key_code: Option<KeyCode>,
        key_state: ButtonState,
    },

    #[doc(hidden)]
    _NonExhaustive,
}