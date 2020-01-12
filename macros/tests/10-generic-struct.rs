use macros::Builder;

#[derive(Builder)]
pub struct Command<T> {
    other: Option<T>,
}

fn main() {
    let command = Command::builder()
        .other(5)
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}