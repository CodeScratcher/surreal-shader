use glium::glutin::{event::Event, self};

pub enum GameEvent {
    Close,
    Left,
    Right,
    Up,
    Down
}

pub fn handle_raw_event(ev: Event<()>) -> Option<GameEvent> {
    match ev {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                return Some(GameEvent::Close);
            },
            _ => return None,
        },
        _ => None,
    }
}