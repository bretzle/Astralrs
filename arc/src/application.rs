use crate::files::Fi;
use macros::Builder;

#[derive(Builder)]
pub struct ApplicationListener {
    pub init: fn(),
    pub resize: fn(width: u16, height: u16),
    pub update: fn(),
    pub pause: fn(),
    pub resume: fn(),
    pub dispose: fn(),
    pub file_dropped: fn(file: &Fi),
}

impl Default for ApplicationListener {
    fn default() -> Self {
        Self {
            init: || {},
            resize: |_,_| {},
            update: || {},
            pause: || {},
            resume: || {},
            dispose: || {},
            file_dropped: |_| {},
        }
    }
}

pub struct ApplicationCore {
    modules: Vec<ApplicationListener>,
    setup: fn(),
}

impl ApplicationCore {
    pub fn new(function: fn()) -> Self {
        ApplicationCore {
            modules: Vec::new(),
            setup: function,
        }
    }

    pub fn add(&mut self, module: ApplicationListener) {
        self.modules.push(module);
    }

    pub fn get_modules(&self) -> &Vec<ApplicationListener> {
        &self.modules
    }

    pub fn init(&self) {
        (self.setup)();

        for module in &self.modules {
            (module.init)()
        }
    }

    pub fn resize(&self, width: u16, height: u16) {
        for module in &self.modules {
            (module.resize)(width, height);
        }
    }

    pub fn update(&self) {
        for module in &self.modules {
            (module.update)();
        }
    }

    pub fn pause(&self) {
        for module in &self.modules {
            (module.pause)();
        }
    }

    pub fn resume(&self) {
        for module in &self.modules {
            (module.resume)();
        }
    }

    pub fn dispose(&self) {
        for module in &self.modules {
            (module.dispose)();
        }
    }

    pub fn file_dropped(&self, file: Fi) {
        for module in &self.modules {
            (module.file_dropped)(&file);
        }
    }
}
