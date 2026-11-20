#![cfg(feature = "serde")]

mod common;

#[derive(Debug, lib_conf::LibConfig)]
#[config(no_autoderive)]
pub struct TestSettings {
    #[config(copy, default = 1)]
    pub foo: u16,
}

#[test]
fn main() {
    let settings = TestSettings::builder()
        .foo(5).build();
    println!("settings: {settings:#?}");

    assert_eq!(settings.foo(), 5);
}
