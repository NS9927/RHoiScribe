use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliCommand {
    Serve,
    Help,
    Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    argument: String,
}

pub fn parse_args<I, S>(args: I) -> Result<CliCommand, CliError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let rest = args
        .into_iter()
        .skip(1)
        .map(|argument| argument.as_ref().to_string())
        .collect::<Vec<_>>();

    match rest.as_slice() {
        [] => Ok(CliCommand::Serve),
        [flag] if flag == "--help" || flag == "-h" => Ok(CliCommand::Help),
        [flag] if flag == "--version" || flag == "-V" => Ok(CliCommand::Version),
        [argument, ..] => Err(CliError {
            argument: argument.clone(),
        }),
    }
}

pub fn version_text() -> String {
    format!("rhoiscribe {}", env!("CARGO_PKG_VERSION"))
}

pub fn help_text() -> &'static str {
    "RHoiScribe - local MCP server for HOI4 Modding agents\n\n\
Usage:\n\
  rhoiscribe            Run the MCP server over stdio\n\
  rhoiscribe --help     Show this help text\n\
  rhoiscribe --version  Show version information\n\n\
MCP clients should launch this binary as a local stdio server.\n"
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown argument `{}`", self.argument)
    }
}

impl Error for CliError {}
