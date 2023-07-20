use winit::event::{VirtualKeyCode, MouseButton, WindowEvent, KeyboardInput, ElementState, MouseScrollDelta};
use std::collections::HashMap;

pub type Key = VirtualKeyCode;
pub type Button = MouseButton;

pub struct Controller {
    key_presses: HashMap<Key, bool>,
    button_presses: HashMap<Button, bool>,

    last_key_presses: HashMap<Key, bool>,
    last_button_presses: HashMap<Button, bool>,
    
    pub mouse_position: (f64, f64),
    pub mouse_wheel: f32
}

impl Controller {
    pub fn new() -> Self {
        Self {
            key_presses: HashMap::new(),
            button_presses: HashMap::new(),
            
            last_key_presses: HashMap::new(),
            last_button_presses: HashMap::new(),

            mouse_position: (0.0, 0.0),
            mouse_wheel: 0.0
        }
    }

    pub fn get_axis(&mut self, negative: Key, positive: Key) -> f32 {
        self.is_key_down(positive) as i32 as f32 - self.is_key_down(negative) as i32 as f32
    }

    pub fn is_key_down(&mut self, keycode: Key) -> bool {
        if let Some(state) = self.key_presses.get(&keycode) {
            if !self.last_key_presses.contains_key(&keycode) {
                self.last_key_presses.insert(keycode, *state);
            } else {
                *self.last_key_presses.get_mut(&keycode).unwrap() = *state;
            }

            *state
        } else {
            false
        }
    }

    pub fn is_key_pressed(&mut self, keycode: Key) -> bool {
        if let Some(state) = self.key_presses.get(&keycode) {
            let last_state = *self.last_key_presses.get(&keycode).unwrap_or(&false);
            
            if !self.last_key_presses.contains_key(&keycode) {
                self.last_key_presses.insert(keycode, *state);
            } else {
                *self.last_key_presses.get_mut(&keycode).unwrap() = *state;
            }

            *state != last_state && *state
        } else {
            false
        }
    }

    pub fn is_button_pressed(&mut self, button: Button) -> bool {
        if let Some(state) = self.button_presses.get(&button) {
            let last_state = *self.last_button_presses.get(&button).unwrap_or(&false);
            
            if !self.last_button_presses.contains_key(&button) {
                self.last_button_presses.insert(button, *state);
            } else {
                *self.last_button_presses.get_mut(&button).unwrap() = *state;
            }

            *state != last_state && *state
        } else {
            false
        }
    }

    pub fn update(&mut self) {
        self.mouse_wheel = 0.0;
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

                if let Some(key_press) = self.key_presses.get_mut(keycode) {
                    *key_press = is_pressed;
                } else {
                    self.key_presses.insert(*keycode, is_pressed);
                }

                true
            }
            WindowEvent::MouseInput { 
                state, 
                button,
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;

                if let Some(button_press) = self.button_presses.get_mut(button) {
                    *button_press = is_pressed;
                } else {
                    self.button_presses.insert(*button, is_pressed);
                }

                true
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