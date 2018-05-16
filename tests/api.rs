#[cfg(test)]
extern crate garden;

mod api {
    use garden::commands::install;

    #[test]
    fn test_install() {
        assert!(install::command(&install::Args {}).is_ok());
    }
}
