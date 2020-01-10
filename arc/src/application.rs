use crate::files::Fi;
use macros::Builder;

// #[derive(Builder)]
pub struct ApplicationListener<T> {
    pub init: for<'m> fn(),
    pub resize: for<'m> fn(width: u16, height: u16),
    pub update: for<'m> fn(),
    pub pause: for<'m> fn(),
    pub resume: for<'m> fn(),
    pub dispose: for<'m> fn(),
    pub file_dropped: for<'m> fn(file: Fi),
    pub injector: Option<T>,
}