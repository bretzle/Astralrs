use fractal::color::*;
use std::u32::{MAX, MIN};

#[test]
fn new() {
    assert_eq!(Color::new(1).0, 1);
    assert_eq!(Color::new(MIN).0, MIN);
    assert_eq!(Color::new(MAX).0, MAX);
}

#[test]
fn default() {
    assert_eq!(Color::default(), (0, 0, 0).into());
}

#[test]
fn from_rgb() {
    assert_eq!(Color::from_rgb(255, 0, 0).0, 0xFF0000);
    assert_eq!(Color::from_rgb(0, 255, 0).0, 0x00FF00);
    assert_eq!(Color::from_rgb(0, 0, 255).0, 0x0000FF);
}

#[test]
fn r() {
    assert_eq!(Color::new(0x336123).r(), 0x33);
}

#[test]
fn g() {
    assert_eq!(Color::new(0x336123).g(), 0x61);
}

#[test]
fn b() {
    assert_eq!(Color::new(0x336123).b(), 0x23);
}

#[test]
fn tuple() {
    assert_eq!(Color::new(0x336123).tuple(), (0x33, 0x61, 0x23));
}

#[test]
fn from() {
    assert_eq!(Color::from(7i32).0, 7);
    assert_eq!(Color::from(7u32).0, 7);
    assert_eq!(Color::from(7u64).0, 7);
}

#[test]
fn named_colors() {
    assert_eq!(SNOW, (255, 250, 250).into());
}
