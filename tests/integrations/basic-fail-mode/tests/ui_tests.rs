use ui_test::*;

fn main() -> ui_test::color_eyre::Result<()> {
    let path = "../../../target";
    let mut config = Config {
        dependencies_crate_manifest_path: Some("Cargo.toml".into()),
        mode: Mode::Fail {
            require_patterns: true,
        },
        ..Config::rustc("tests/actual_tests".into())
    };
    if std::env::var_os("BLESS").is_some() {
        config.output_conflict_handling = OutputConflictHandling::Bless
    }
    config.stderr_filter("in ([0-9]m )?[0-9\\.]+s", "");
    config.stdout_filter("in ([0-9]m )?[0-9\\.]+s", "");
    config.stderr_filter(r"[^ ]*/\.?cargo/registry/.*/", "$$CARGO_REGISTRY");
    config.path_stderr_filter(&std::path::Path::new(path), "$DIR");

    run_tests_generic(
        config,
        Args::default(),
        default_file_filter,
        default_per_file_config,
        // Avoid github actions, as these would end up showing up in `Cargo.stderr`
        status_emitter::Text::verbose(),
    )
}
