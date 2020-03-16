use crate::gl::math::Mat4;
use glium::glutin::{dpi::PhysicalPosition, event};
use std::collections::HashSet;

pub struct Camera {
    pub view: Mat4,
    last_cursor_position: PhysicalPosition<f64>,
    left_button_state: event::ElementState,
    modifiers_state: event::ModifiersState,
    key_state: HashSet<event::VirtualKeyCode>,
    last_time: std::time::Instant,
}

impl Camera {
    pub fn new(view: Mat4) -> Camera {
        Camera {
            view: view,
            last_cursor_position: PhysicalPosition::new(0.0, 0.0),
            left_button_state: event::ElementState::Released,
            modifiers_state: event::ModifiersState::empty(),
            key_state: HashSet::new(),
            last_time: std::time::Instant::now(),
        }
    }
    pub fn update<T>(&mut self, event: &event::Event<T>) {
        match &event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CursorMoved { position, .. } => {
                    if self.left_button_state == event::ElementState::Pressed {
                        let dx = (position.x - self.last_cursor_position.x) as f32;
                        let dy = (position.y - self.last_cursor_position.y) as f32;
                        self.view = Mat4::rotation(-0.01 * dy, 1.0, 0.0, 0.0) * self.view;
                        self.view = Mat4::rotation(-0.01 * dx, 0.0, 1.0, 0.0) * self.view;
                    }
                    self.last_cursor_position = *position;
                }
                event::WindowEvent::MouseInput {
                    state,
                    button: event::MouseButton::Left,
                    ..
                } => {
                    self.left_button_state = *state;
                }
                event::WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => {
                    if let Some(key) = virtual_keycode {
                        match state {
                            event::ElementState::Pressed => {
                                self.key_state.insert(*key);
                            }
                            event::ElementState::Released => {
                                self.key_state.remove(key);
                            }
                        }
                    }
                }
                _ => (),
            },
            event::Event::DeviceEvent { event, .. } => match event {
                event::DeviceEvent::ModifiersChanged(state) => {
                    self.modifiers_state = *state;
                }
                _ => (),
            },
            _ => (),
        }

        let v = if self.modifiers_state.shift() {
            10.0
        } else {
            1.0
        };

        let d = v * (std::time::Instant::now() - self.last_time).as_secs_f32();
        self.last_time = std::time::Instant::now();

        if self.key_state.contains(&event::VirtualKeyCode::W) {
            self.view = Mat4::translation(0.0, 0.0, d) * self.view;
        }
        if self.key_state.contains(&event::VirtualKeyCode::S) {
            self.view = Mat4::translation(0.0, 0.0, -d) * self.view;
        }
        if self.key_state.contains(&event::VirtualKeyCode::A) {
            self.view = Mat4::translation(d, 0.0, 0.0) * self.view;
        }
        if self.key_state.contains(&event::VirtualKeyCode::D) {
            self.view = Mat4::translation(-d, 0.0, 0.0) * self.view;
        }
        if self.key_state.contains(&event::VirtualKeyCode::R) {
            self.view = Mat4::translation(0.0, -d, 0.0) * self.view;
        }
        if self.key_state.contains(&event::VirtualKeyCode::F) {
            self.view = Mat4::translation(0.0, d, 0.0) * self.view;
        }
        if self.key_state.contains(&event::VirtualKeyCode::Q) {
            self.view = Mat4::rotation(0.5 * d, 0.0, 0.0, 1.0) * self.view;
        }
        if self.key_state.contains(&event::VirtualKeyCode::E) {
            self.view = Mat4::rotation(-0.5 * d, 0.0, 0.0, 1.0) * self.view;
        }
    }
}
