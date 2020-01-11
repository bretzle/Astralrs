use crate::error::GameResult;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FullscreenType {
    Windowed,
    True,
    Desktop,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowMode {
    pub width: f32,
    pub height: f32,
    pub maximized: bool,
    pub fullscreen_type: FullscreenType,
    pub borderless: bool,
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub resizeable: bool,
}

impl Default for WindowMode {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizeable: false,
        }
    }
}

impl WindowMode {
    pub fn dimensions(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    pub fn fullscreen_type(mut self, fullscreen_type: FullscreenType) -> Self {
        self.fullscreen_type = fullscreen_type;
        self
    }

    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    pub fn min_dimensions(mut self, width: f32, height: f32) -> Self {
        self.min_width = width;
        self.min_height = height;
        self
    }

    pub fn max_dimensions(mut self, width: f32, height: f32) -> Self {
        self.max_width = width;
        self.max_height = height;
        self
    }

    pub fn resizable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowSetup {
    pub title: String,
    pub samples: NumSamples,
    pub vsync: bool,
    pub icon: String,
    pub srgb: bool,
}

impl Default for WindowSetup {
    fn default() -> Self {
        Self {
            title: String::from("A good game"),
            samples: NumSamples::Zero,
            vsync: true,
            icon: String::new(),
            srgb: true,
        }
    }
}

/// Number of samples for anti-aliasing
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NumSamples {
    Zero = 0,
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}

impl NumSamples {
    pub fn from_u32(i: u32) -> Option<NumSamples> {
        match i {
            0 => Some(NumSamples::Zero),
            1 => Some(NumSamples::One),
            2 => Some(NumSamples::Two),
            4 => Some(NumSamples::Four),
            8 => Some(NumSamples::Eight),
            16 => Some(NumSamples::Sixteen),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Backend {
    OpenGL { major: u8, minor: u8 },
    OpenGLES { major: u8, minor: u8 },
}

impl Default for Backend {
    fn default() -> Self {
        Self::OpenGLES { major: 3, minor: 0 };
        Self::OpenGL { major: 3, minor: 2 }
    }
}

impl Backend {
    pub fn version(self, new_major: u8, new_minor: u8) -> Self {
        match self {
            Backend::OpenGL { .. } => Backend::OpenGL {
                major: new_major,
                minor: new_minor,
            },
            Backend::OpenGLES { .. } => Backend::OpenGLES {
                major: new_major,
                minor: new_minor,
            },
        }
    }

    pub fn gl(self) -> Self {
        match self {
            Backend::OpenGLES { major, minor } => Backend::OpenGL { major, minor },
            gl => gl,
        }
    }

    pub fn gles(self) -> Self {
        match self {
            Backend::OpenGL { major, minor } => Backend::OpenGLES { major, minor },
            es => es,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct ModuleConfig {
    pub gamepad: bool,
    pub audio: bool,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            gamepad: true,
            audio: true,
        }
    }
}

impl ModuleConfig {
    /// Sets whether or not to enable the gamepad input module.
    pub fn gamepad(mut self, gamepad: bool) -> Self {
        self.gamepad = gamepad;
        self
    }

    /// Sets whether or not to enable the audio module.
    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = audio;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Config {
    pub window_mode: WindowMode,
    pub window_setup: WindowSetup,
    pub backend: Backend,
    pub modules: ModuleConfig,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the window mode
    pub fn window_mode(mut self, window_mode: WindowMode) -> Self {
        self.window_mode = window_mode;
        self
    }

    /// Sets the backend
    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    /// Sets the backend
    pub fn modules(mut self, modules: ModuleConfig) -> Self {
        self.modules = modules;
        self
    }

    pub fn to_toml(&self, file: &mut dyn std::io::Write) -> GameResult {
        let s = toml::to_vec(self)?;
        file.write_all(&s)?;
        Ok(())
    }

    pub fn from_toml(file: &mut dyn std::io::Read) -> GameResult<Config> {
        let mut s = String::new();
        let _ = file.read_to_string(&mut s)?;
        let decoded = toml::from_str(&s)?;
        Ok(decoded)
    }

}