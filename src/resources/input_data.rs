use super::*;

pub struct InputData {
    pub input_events: HashMap<legion::Entity, (InputEvent, Time)>,

    pub frame_data: (InputFrameData, InputFrameData),

    pub dragged_entity: Option<legion::Entity>,
    pub hovered_entity: Option<legion::Entity>,

    pub down_keys: HashSet<geng::Key>,
    pub pressed_keys: HashSet<geng::Key>,
    pub down_mouse_buttons: HashSet<geng::MouseButton>,
    pub up_mouse_buttons: HashSet<geng::MouseButton>,
    pub pressed_mouse_buttons: HashSet<geng::MouseButton>,
    pub mouse_world_pos: vec2<f32>,
    pub mouse_screen_pos: vec2<f32>,
}

#[derive(Clone)]
pub struct InputFrameData {
    pub attention: Option<legion::Entity>,
    pub mouse: vec2<f32>,
    pub state: InputState,
}

impl InputFrameData {
    pub fn is_dragged(&self, entity: legion::Entity) -> bool {
        self.state == InputState::Drag && self.attention.unwrap() == entity
    }
    pub fn is_hovered(&self, entity: legion::Entity) -> bool {
        self.state == InputState::Hover && self.attention.unwrap() == entity
    }
    pub fn get_hovered(&self) -> Option<legion::Entity> {
        match self.state {
            InputState::Hover => Some(self.attention.unwrap()),
            _ => None,
        }
    }
    pub fn get_dragged(&self) -> Option<legion::Entity> {
        match self.state {
            InputState::Drag => Some(self.attention.unwrap()),
            _ => None,
        }
    }
}

pub type Handler = fn(InputEvent, legion::Entity, &mut Shader, &mut legion::World, &mut Resources);

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum InputState {
    None,
    Hover,
    Press,
    Click,
    Drag,
}

impl Default for InputState {
    fn default() -> Self {
        Self::None
    }
}

impl Default for InputFrameData {
    fn default() -> Self {
        Self {
            mouse: vec2::ZERO,
            attention: default(),
            state: default(),
        }
    }
}

impl Default for InputData {
    fn default() -> Self {
        Self {
            frame_data: default(),
            down_keys: default(),
            pressed_keys: default(),
            down_mouse_buttons: default(),
            up_mouse_buttons: default(),
            pressed_mouse_buttons: default(),
            mouse_world_pos: vec2::ZERO,
            mouse_screen_pos: vec2::ZERO,
            input_events: default(),
            dragged_entity: default(),
            hovered_entity: default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    HoverStart,
    Hover,
    HoverStop,
    DragStart,
    Drag { delta: vec2<f32> },
    DragStop,
    PressStart,
    Press,
    PressStop,
    Click,
}