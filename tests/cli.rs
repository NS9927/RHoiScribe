use rhoiscribe::cli::{CliCommand, help_text, parse_args, version_text};

#[test]
fn parse_version_flag() {
    let command = parse_args(["rhoiscribe", "--version"]).expect("version flag should parse");

    assert_eq!(command, CliCommand::Version);
}

#[test]
fn parse_help_flag() {
    let command = parse_args(["rhoiscribe", "--help"]).expect("help flag should parse");

    assert_eq!(command, CliCommand::Help);
}

#[test]
fn default_command_serves_stdio() {
    let command = parse_args(["rhoiscribe"]).expect("default command should parse");

    assert_eq!(command, CliCommand::Serve);
}

#[test]
fn help_mentions_mcp_stdio() {
    let help = help_text();

    assert!(help.contains("MCP"));
    assert!(help.contains("stdio"));
    assert!(help.contains("--version"));
}

#[test]
fn version_text_uses_package_version() {
    assert_eq!(
        version_text(),
        format!("rhoiscribe {}", env!("CARGO_PKG_VERSION"))
    );
}
