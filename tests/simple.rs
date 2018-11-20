#[macro_use]
extern crate optional_struct;

#[derive(OptionalStruct, Clone)]
#[optional_derive(Clone)]
struct Config {
    delay: Option<u32>,
    path: String,
    percentage: f32,
}

#[test]
fn test_apply_options() {
    let mut config_apply = Config {
        delay: Some(2),
        path: "/var/log/foo.log".to_owned(),
        percentage: 3.12,
    };
    let config_with = config_apply.clone();

    let opt_config_apply = OptionalConfig {
        delay: None,
        path: Some("/tmp/bar.log".to_owned()),
        percentage: Some(42.24),
    };
    let opt_config_with = opt_config_apply.clone();

    config_apply.apply_options(opt_config_apply);
    let config_with = config_with.with_options(opt_config_with);

    run_check(config_apply);
    run_check(config_with);
}

fn run_check(config: Config) {
    assert_eq!(config.delay, None);
    assert_eq!(config.path, "/tmp/bar.log");
    assert_eq!(config.percentage, 42.24);
}
