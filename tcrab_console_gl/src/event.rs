pub fn translate(glutin_event: glutin::Event) -> Option<tcrab_console::Event> {
    use tcrab_console::Event;
    Some(match glutin_event {
        glutin::Event::WindowEvent { event: glutin_window_event, .. } => {
            match glutin_window_event {
                glutin::WindowEvent::CloseRequested => Event::Quit,
                _ => return None,
            }
        },
        _ => return None,
    })
}