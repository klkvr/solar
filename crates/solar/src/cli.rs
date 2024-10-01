use clap::{ColorChoice, Parser, ValueHint};
use solar_config::{CompilerOutput, CompilerStage, EvmVersion, Language};
use std::path::PathBuf;

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

/// Blazingly fast Solidity compiler.
#[derive(Parser)]
#[command(
    name = "solar",
    version = VERSION_MESSAGE,
    // after_help = "Find more information in the book: http://book.getfoundry.sh/reference/forge/forge.html",
    next_display_order = None,
)]
pub struct Args {
    /// Files to compile or import remappings.
    #[arg(value_hint = ValueHint::FilePath)]
    pub input: Vec<PathBuf>,
    /// Directory to search for files.
    #[arg(long, short = 'I', visible_alias = "base-path")]
    pub import_path: Vec<PathBuf>,
    /// Map to search for files [format: map=path]
    #[arg(long, short = 'm')]
    pub import_map: Vec<ImportMap>,
    /// Source code language.
    #[arg(long, value_enum, default_value_t)]
    pub language: Language,
    /// EVM version.
    #[arg(long, value_enum, default_value_t)]
    pub evm_version: EvmVersion,
    /// Stop execution after the given compiler stage.
    #[arg(long, value_enum)]
    pub stop_after: Option<CompilerStage>,

    /// Coloring.
    #[arg(long, value_enum, default_value_t)]
    pub color: ColorChoice,
    /// Use verbose output.
    #[arg(long, short)]
    pub verbose: bool,
    /// Pretty-print any JSON output.
    #[arg(long, short)]
    pub pretty_json: bool,
    /// How errors and other messages are produced.
    #[arg(long, value_enum, default_value_t)]
    pub error_format: ErrorFormat,
    /// Number of threads to use. Zero specifies the number of logical cores.
    // TODO: Default to `0` once we're actually using rayon.
    #[arg(long, short = 'j', visible_alias = "jobs", default_value = "1")]
    pub threads: usize,
    /// Comma separated list of types of output for the compiler to emit.
    #[arg(long)]
    pub emit: Vec<CompilerOutput>,

    /// Unstable flags. WARNING: these are completely unstable, and may change at any time.
    ///
    /// See `-Z help` for more details.
    // TODO: `-Zhelp` needs positional arg, and also it's displayed like a normal command.
    // TODO: Figure out if we can flatten this directly in clap derives.
    #[doc(hidden)]
    #[arg(id = "unstable-features", value_name = "FLAG", short = 'Z')]
    pub _unstable: Vec<String>,

    /// Parsed unstable flags.
    #[arg(skip)]
    pub unstable: UnstableFeatures,
}

impl Args {
    pub(crate) fn populate_unstable(&mut self) -> Result<(), clap::Error> {
        if !self._unstable.is_empty() {
            let hack = self._unstable.iter().map(|s| format!("--{s}"));
            self.unstable =
                UnstableFeatures::try_parse_from(std::iter::once(String::new()).chain(hack))?;
        }
        Ok(())
    }
}

/// How errors and other messages are produced.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ErrorFormat {
    #[default]
    Human,
    Json,
    RichJson,
}

#[derive(Clone, Debug)]
pub struct ImportMap {
    pub map: PathBuf,
    pub path: PathBuf,
}

impl std::str::FromStr for ImportMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once('=') {
            Ok(Self { map: a.into(), path: b.into() })
        } else {
            Err("missing '='")
        }
    }
}

/// Internal options.
#[derive(Clone, Debug, Default, Parser)]
pub struct UnstableFeatures {
    /// Enables UI testing mode.
    #[arg(long)]
    pub ui_testing: bool,
    /// Prints a note for every diagnostic that is emitted with the creation and emission location.
    ///
    /// This is enabled by default on debug builds.
    #[arg(long)]
    pub track_diagnostics: bool,
    /// Enables parsing Yul files for testing.
    #[arg(long)]
    pub parse_yul: bool,

    #[cfg(test)]
    #[arg(long)]
    test_bool: bool,
    #[cfg(test)]
    #[arg(long)]
    test_value: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert();
        UnstableFeatures::command().debug_assert();
    }

    #[test]
    fn unstable_features() {
        fn parse(args: &[&str]) -> Result<UnstableFeatures, impl std::fmt::Debug> {
            struct UnwrapDisplay<T>(T);
            impl<T: std::fmt::Display> std::fmt::Debug for UnwrapDisplay<T> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "\n{}", self.0)
                }
            }
            (|| {
                let mut args = Args::try_parse_from(args)?;
                args.populate_unstable()?;
                Ok::<_, clap::Error>(args.unstable)
            })()
            .map_err(|e| UnwrapDisplay(e.render().ansi().to_string()))
        }

        let unstable = parse(&["solar", "a.sol"]).unwrap();
        assert!(!unstable.test_bool);

        let unstable = parse(&["solar", "-Ztest-bool", "a.sol"]).unwrap();
        assert!(unstable.test_bool);
        let unstable = parse(&["solar", "-Z", "test-bool", "a.sol"]).unwrap();
        assert!(unstable.test_bool);

        assert!(parse(&["solar", "-Ztest-value", "a.sol"]).is_err());
        assert!(parse(&["solar", "-Z", "test-value", "a.sol"]).is_err());
        assert!(parse(&["solar", "-Ztest-value", "2", "a.sol"]).is_err());
        let unstable = parse(&["solar", "-Ztest-value=2", "a.sol"]).unwrap();
        assert_eq!(unstable.test_value, Some(2));
        let unstable = parse(&["solar", "-Z", "test-value=2", "a.sol"]).unwrap();
        assert_eq!(unstable.test_value, Some(2));
    }
}