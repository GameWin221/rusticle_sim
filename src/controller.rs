use winit::{event::{WindowEvent, ElementState, VirtualKeyCode, KeyboardInput, MouseScrollDelta}};
use std::collections::HashMap;

pub type Key = VirtualKeyCode;

pub struct Controller {
    key_presses: HashMap<Key, bool>,
    
    pub mouse_position: (f64, f64),
    pub mouse_wheel: f32
}

impl Controller {
    pub fn new() -> Self {
        let key_presses = HashMap::new();

        Self {
            key_presses,
            mouse_position: (0.0, 0.0),
            mouse_wheel: 0.0
        }
    }

    pub fn get_axis(&self, negative: Key, positive: Key) -> f32 {
        self.is_key_pressed(positive) as i32 as f32 - self.is_key_pressed(negative) as i32 as f32
    }

    pub fn is_key_pressed(&self, keycode: Key) -> bool {
        if self.key_presses.contains_key(&keycode) {
            *self.key_presses.get(&keycode).unwrap()
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

                if !self.key_presses.contains_key(keycode) {
                    self.key_presses.insert(*keycode, false);
                } 

                *self.key_presses.get_mut(keycode).unwrap() = is_pressed;

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