//! Virtual touch controls for mobile/Android (Pojav / Mojo Launcher layout).
//!
//! Maps screen touch events to game actions (D-Pad, Jump, Sneak, Sprint, Attack,
//! Use, Inventory, Chat, Pause, and Camera drag).
#![forbid(unsafe_code)]

use std::collections::HashMap;

/// Identifies an on-screen touch control button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirtualButton {
    // Movement D-pad
    Forward,
    Backward,
    Left,
    Right,

    // Actions
    Jump,
    Sneak,
    Sprint,
    Attack,     // Left click
    UseItem,    // Right click
    PickBlock,  // Middle click

    // Interface
    Inventory,
    DropItem,
    Chat,
    PauseMenu,
    DebugF3,
    PerspectiveF5,
}

/// A 2D bounding box in normalized screen coordinates [0.0, 1.0].
#[derive(Debug, Clone, Copy)]
pub struct NormalizedRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl NormalizedRect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= (self.x + self.width) && py >= self.y && py <= (self.y + self.height)
    }
}

/// Complete configuration and state for virtual mobile touch controls.
#[derive(Debug, Clone)]
pub struct VirtualControlsOverlay {
    /// Mapping from virtual button to its normalized screen region.
    pub buttons: HashMap<VirtualButton, NormalizedRect>,
    /// Active finger touch IDs bound to virtual buttons.
    pub active_button_touches: HashMap<u64, VirtualButton>,
    /// Active touch ID assigned to camera rotation/look drag.
    pub camera_touch_id: Option<u64>,
    /// Last recorded position of the camera touch (normalized x, y).
    pub last_camera_pos: Option<(f32, f32)>,
    /// Accumulated camera look delta (dx, dy) for the current frame.
    pub camera_delta: (f32, f32),
}

impl Default for VirtualControlsOverlay {
    fn default() -> Self {
        Self::pojav_default_layout()
    }
}

impl VirtualControlsOverlay {
    /// Pojav/Mojo-inspired default layout on widescreen mobile displays (16:9 / 20:9).
    pub fn pojav_default_layout() -> Self {
        let mut buttons = HashMap::new();

        // D-pad on bottom-left:
        buttons.insert(VirtualButton::Forward, NormalizedRect::new(0.08, 0.65, 0.08, 0.12));
        buttons.insert(VirtualButton::Backward, NormalizedRect::new(0.08, 0.85, 0.08, 0.12));
        buttons.insert(VirtualButton::Left, NormalizedRect::new(0.02, 0.75, 0.06, 0.12));
        buttons.insert(VirtualButton::Right, NormalizedRect::new(0.16, 0.75, 0.06, 0.12));
        buttons.insert(VirtualButton::Sneak, NormalizedRect::new(0.09, 0.77, 0.06, 0.08));

        // Jump on bottom-right:
        buttons.insert(VirtualButton::Jump, NormalizedRect::new(0.88, 0.72, 0.09, 0.16));

        // Attack (Left Click) & Use (Right Click) on the sides:
        buttons.insert(VirtualButton::Attack, NormalizedRect::new(0.77, 0.65, 0.08, 0.14));
        buttons.insert(VirtualButton::UseItem, NormalizedRect::new(0.77, 0.80, 0.08, 0.14));

        // Top bar buttons:
        buttons.insert(VirtualButton::PauseMenu, NormalizedRect::new(0.46, 0.02, 0.08, 0.08));
        buttons.insert(VirtualButton::Chat, NormalizedRect::new(0.56, 0.02, 0.06, 0.08));
        buttons.insert(VirtualButton::Inventory, NormalizedRect::new(0.64, 0.02, 0.06, 0.08));
        buttons.insert(VirtualButton::DebugF3, NormalizedRect::new(0.02, 0.02, 0.06, 0.06));
        buttons.insert(VirtualButton::PerspectiveF5, NormalizedRect::new(0.10, 0.02, 0.06, 0.06));
        buttons.insert(VirtualButton::DropItem, NormalizedRect::new(0.72, 0.02, 0.06, 0.08));

        Self {
            buttons,
            active_button_touches: HashMap::new(),
            camera_touch_id: None,
            last_camera_pos: None,
            camera_delta: (0.0, 0.0),
        }
    }

    /// Handles a touch down event at normalized coordinates `(x, y)` in `[0.0, 1.0]`.
    pub fn on_touch_down(&mut self, touch_id: u64, x: f32, y: f32) {
        // Check if touch hit any virtual button:
        for (&button, &rect) in &self.buttons {
            if rect.contains(x, y) {
                self.active_button_touches.insert(touch_id, button);
                return;
            }
        }

        // If not a button and on right side of screen, treat as camera swipe:
        if x > 0.35 && self.camera_touch_id.is_none() {
            self.camera_touch_id = Some(touch_id);
            self.last_camera_pos = Some((x, y));
        }
    }

    /// Handles a touch move event at normalized coordinates `(x, y)`.
    pub fn on_touch_move(&mut self, touch_id: u64, x: f32, y: f32) {
        if Some(touch_id) == self.camera_touch_id {
            if let Some((last_x, last_y)) = self.last_camera_pos {
                self.camera_delta.0 += x - last_x;
                self.camera_delta.1 += y - last_y;
            }
            self.last_camera_pos = Some((x, y));
        }
    }

    /// Handles a touch up or cancelled event.
    pub fn on_touch_up(&mut self, touch_id: u64) {
        self.active_button_touches.remove(&touch_id);
        if Some(touch_id) == self.camera_touch_id {
            self.camera_touch_id = None;
            self.last_camera_pos = None;
        }
    }

    /// Checks if a particular virtual button is currently pressed.
    pub fn is_pressed(&self, button: VirtualButton) -> bool {
        self.active_button_touches.values().any(|&b| b == button)
    }

    /// Computes keyboard movement vector [x, z] from the touch D-Pad state.
    pub fn movement_vector(&self) -> [f32; 2] {
        let forward = self.is_pressed(VirtualButton::Forward);
        let backward = self.is_pressed(VirtualButton::Backward);
        let left = self.is_pressed(VirtualButton::Left);
        let right = self.is_pressed(VirtualButton::Right);

        crate::input::keyboard_direction(forward, backward, left, right)
    }

    /// Consumes and resets the accumulated camera look delta.
    pub fn take_camera_delta(&mut self) -> (f32, f32) {
        let delta = self.camera_delta;
        self.camera_delta = (0.0, 0.0);
        delta
    }
}
