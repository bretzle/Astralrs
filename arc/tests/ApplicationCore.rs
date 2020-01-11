use arc::application::{ApplicationCore, ApplicationListener};

#[test]
fn test_adding_modules() {
    let mut core = ApplicationCore::new(|| {});

    core.add(ApplicationListener::default());

    assert_eq!(core.get_modules().len(), 1);
}