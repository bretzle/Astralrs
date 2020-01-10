pub mod application;
pub mod files;

use application::ApplicationListener;

#[test]
fn it_works() {
    let _listener = ApplicationListener::builder()
        .init(|| {})
        .resize(|_, _| {})
        .update(|| {})
        .pause(|| {})
        .resume(|| {})
        .dispose(|| {})
        .file_dropped(|_| {})
        .build()
        .unwrap();
}
