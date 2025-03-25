use crate::utils::array_key;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

macro_rules! make_folder {
    (
        $struct_name: ident :
        $(
            $attr_name: ident
            =
            $default: expr
        );* $(;)?
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(
                pub $attr_name: KeyBind,
            )*
        }
        impl $struct_name {
            fn new() -> Self {
                Self {
                    $(
                        $attr_name: $default,
                    )*
                }
            }
            fn bind_map(&mut self, f: &impl Fn(&mut KeyBind)) {
                $(
                    f(&mut self.$attr_name);
                )*
            }
        }
    };
}

make_folder!(CameraChanges:
    next_cam = KeyBind::new(PressState::Pressed, vec![KeyCode::Semicolon]);
    prev_cam = KeyBind::new(PressState::Pressed, vec![KeyCode::KeyL]);
    rng_cam = KeyBind::new(PressState::Pressed, vec![KeyCode::KeyP]);
    reset_cam = KeyBind::new(PressState::Pressed, vec![KeyCode::KeyO]);
    reset_pos = KeyBind::new(PressState::Pressed, vec![KeyCode::KeyR]);
    toggle_lock = KeyBind::new(PressState::Pressed, vec![KeyCode::KeyU]);
);

make_folder!(WindowDebug:
    show_fps = KeyBind::new(PressState::Active, vec![KeyCode::F3, KeyCode::KeyX])
);

array_key!(
    pub enum MoveKey {
        Left,
        Right,
        Up,
        Down,
        Forward,
        Backward,
    }
);
#[derive(Debug, Clone)]
struct PressResume {
    pub any_pressed: bool,
    pub any_released: bool,
    pub all_active: bool,
    pub all_inactive: bool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum PressState {
    #[default]
    Inactive,
    Pressed,
    Active,
    Released,
}
impl PressState {
    fn next(&mut self) {
        *self = match self {
            Self::Inactive => Self::Inactive,
            Self::Pressed => Self::Active,
            Self::Active => Self::Active,
            Self::Released => Self::Inactive,
        }
    }
    fn add_on_resume(&self, resume: &mut PressResume) {
        resume.any_pressed |= matches!(self, Self::Pressed);
        resume.any_released |= matches!(self, Self::Released);
        resume.all_active &= matches!(self, Self::Active | Self::Pressed | Self::Released);
        resume.all_inactive &= matches!(self, Self::Inactive);
    }
}
impl From<ElementState> for PressState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => Self::Pressed,
            ElementState::Released => Self::Released,
        }
    }
}

#[derive(Debug, Clone)]
struct KeyState {
    code: KeyCode,
    press_state: PressState,
}
impl KeyState {
    fn new(code: KeyCode) -> Self {
        Self {
            code,
            press_state: PressState::default(),
        }
    }
    fn process(&mut self, key: KeyCode, state: ElementState) {
        if key != self.code {
            return;
        }
        self.press_state = state.into();
    }
    fn next_frame(&mut self) {
        self.press_state.next();
    }
    fn add_on_resume(&self, resume: &mut PressResume) {
        self.press_state.add_on_resume(resume)
    }
}

#[derive(Debug, Clone)]
pub struct KeyBind {
    keys: Vec<KeyState>,
    trigger: PressState,
}
impl KeyBind {
    fn new(trigger: PressState, keys: Vec<KeyCode>) -> Self {
        Self {
            keys: keys.into_iter().map(|k| KeyState::new(k)).collect(),
            trigger,
        }
    }
    fn process(&mut self, key: KeyCode, state: ElementState) {
        for k in &mut self.keys {
            k.process(key, state)
        }
    }
    fn next_frame(&mut self) {
        for k in &mut self.keys {
            k.next_frame()
        }
    }
    pub fn is_active(&self) -> bool {
        let mut resume = PressResume {
            any_pressed: false,
            any_released: false,
            all_active: true,
            all_inactive: true,
        };
        for k in &self.keys {
            k.add_on_resume(&mut resume);
        }
        match self.trigger {
            PressState::Pressed => resume.any_pressed & resume.all_active,
            PressState::Active => resume.all_active,
            PressState::Released => resume.any_released & resume.all_active,
            PressState::Inactive => resume.all_inactive,
        }
    }
}

pub struct KeyBinds {
    pub camera_moves: [KeyBind; MoveKey::COUNT],
    pub camera_change: CameraChanges,
    pub window_debug: WindowDebug,
}
impl KeyBinds {
    fn bind_map(&mut self, f: &impl Fn(&mut KeyBind)) {
        for b in &mut self.camera_moves {
            f(b)
        }
        self.camera_change.bind_map(f);
        self.window_debug.bind_map(f);
    }
    pub fn process(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    repeat: false,
                    ..
                },
            ..
        } = event
        {
            self.bind_map(&|b| b.process(*code, *state))
        }
        if let WindowEvent::RedrawRequested = event {
            self.next_frame();
        }
    }
    pub fn next_frame(&mut self) {
        self.bind_map(&|b| b.next_frame());
    }
    pub fn base_binds() -> Self {
        Self {
            camera_moves: MoveKey::ARRAY.map(|k| {
                KeyBind::new(PressState::Active, vec![match k {
                    MoveKey::Left => KeyCode::KeyA,
                    MoveKey::Right => KeyCode::KeyD,
                    MoveKey::Backward => KeyCode::KeyS,
                    MoveKey::Forward => KeyCode::KeyW,
                    MoveKey::Down => KeyCode::KeyQ,
                    MoveKey::Up => KeyCode::KeyE,
                }])
            }),
            camera_change: CameraChanges::new(),
            window_debug: WindowDebug::new(),
        }
    }
}
