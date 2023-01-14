use winit::{event::{WindowEvent, ElementState, VirtualKeyCode, KeyboardInput, MouseScrollDelta}};

pub struct Controller {
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_escape_pressed: bool,
    pub is_space_pressed: bool,
    pub is_r_pressed: bool,
    pub is_t_pressed: bool,
    pub mouse_position: (f64, f64),
    pub mouse_wheel: f32
}

impl Controller {
    pub fn new() -> Self {
        Self {
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_escape_pressed: false,
            is_space_pressed: false,
            is_r_pressed: false,
            is_t_pressed: false,
            mouse_position: (0.0, 0.0),
            mouse_wheel: 0.0
        }
    }

    pub fn process_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;

                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Escape => {
                        self.is_escape_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.is_space_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::R => {
                        self.is_r_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::T => {
                        self.is_t_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::CursorMoved { 
                position,
                ..
            } => {
                self.mouse_position = (position.x, position.y);
                true
            },
            WindowEvent::MouseWheel { 
                delta,
                ..
            } => {

                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.mouse_wheel = *y;
                    }
                    _ => {}
                }

                true
            },
            _ => false,
        }
    }
}