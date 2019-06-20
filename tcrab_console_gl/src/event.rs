use tcrab_console::event::{Event, KeyCode, ButtonState};

fn translate_key_code(glutin_key_code: glutin::VirtualKeyCode) -> Option<KeyCode> {
    Some(match glutin_key_code {
        glutin::VirtualKeyCode::Escape => KeyCode::Escape,
        glutin::VirtualKeyCode::Up => KeyCode::Up,
        glutin::VirtualKeyCode::Down => KeyCode::Down,
        glutin::VirtualKeyCode::Left => KeyCode::Left,
        glutin::VirtualKeyCode::Right => KeyCode::Right,
        _ => return None,
    })
}

pub fn translate(glutin_event: glutin::Event) -> Option<Event> {
    Some(match glutin_event {
        glutin::Event::WindowEvent { event: glutin_window_event, .. } => {
            match glutin_window_event {
                glutin::WindowEvent::CloseRequested => Event::Quit,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    let key_state = match input.state {
                        glutin::ElementState::Pressed => ButtonState::Pressed,
                        glutin::ElementState::Released => ButtonState::Released,
                    };
                    Event::KeyboardInput {
                        key_code: input.virtual_keycode.and_then(translate_key_code),
                        key_state,
                    }
                }
                _ => return None,
            }
        },
        _ => return None,
    })
}