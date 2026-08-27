use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use okf::{InspectKind, KnowledgeFilter};

use crate::{
    CheckFormat, ProfileArg, TrustTierArg, benchmark, check, inspect, print_check, search,
};

#[derive(Parser)]
#[command(
    name = "okmate",
    version,
    about = "OKMate (open knowledge mate) — Askama + Axum knowledge application over the portable okf engine",
    arg_required_else_help = true,
    subcommand_required = true
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate an OKF bundle without writing output.
    Check {
        #[arg(default_value = "knowledge")]
        root: PathBuf,
        #[arg(long, value_enum, default_value_t = ProfileArg::Strict)]
        profile: ProfileArg,
        #[arg(long, value_enum, default_value_t = CheckFormat::Terminal)]
        format: CheckFormat,
    },
    /// Print normalized concepts or the bundle graph as JSON.
    Inspect {
        #[command(subcommand)]
        target: InspectTarget,
        #[arg(long, value_enum, default_value_t = ProfileArg::Strict)]
        profile: ProfileArg,
    },
    /// Search metadata and heading chunks as JSON.
    Search {
        query: String,
        #[arg(default_value = "knowledge")]
        root: PathBuf,
        #[arg(long, value_enum, default_value_t = ProfileArg::Strict)]
        profile: ProfileArg,
        #[command(flatten)]
        filters: FiltersArg,
    },
    /// Measure local load, site, and click spans (machine-local, not an SLA).
    Timings {
        /// Knowledge bundle directory or a Markdown file inside one.
        path: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = CheckFormat::Terminal)]
        format: CheckFormat,
        #[arg(long, value_enum, default_value_t = TimingsScenarioArg::All)]
        scenario: TimingsScenarioArg,
        #[arg(long, value_enum, default_value_t = ProfileArg::Strict)]
        profile: ProfileArg,
        /// Override preview provenance (default matches `LoadOptions` for `--profile`).
        #[arg(long, overrides_with = "no_provenance")]
        provenance: bool,
        #[arg(long = "no-provenance", overrides_with = "provenance")]
        no_provenance: bool,
    },
    /// Run a retrieval benchmark TOML file against a knowledge bundle.
    Benchmark {
        benchmark: PathBuf,
        #[arg(default_value = "knowledge")]
        root: PathBuf,
        #[arg(long, value_enum, default_value_t = ProfileArg::Strict)]
        profile: ProfileArg,
    },
    /// Preview an OKF bundle with live reload.
    View {
        /// Knowledge bundle directory or a Markdown file inside one.
        path: Option<PathBuf>,
        /// Write preview output here instead of a temp directory.
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = ProfileArg::Strict)]
        profile: ProfileArg,
        /// Skip the preview window; print the URL and keep serving.
        #[arg(long)]
        no_window: bool,
        /// Bind every interface (`0.0.0.0`). Default is localhost only.
        #[arg(long)]
        public: bool,
        /// TCP port. Defaults to a free port with the preview window, or 8000
        /// with `--no-window`. Pass `auto` to pick a free port.
        #[arg(
            long,
            default_value = "auto",
            default_value_if("no_window", "true", "8000"),
            value_name = "PORT",
            value_parser = crate::port::parse_port_arg
        )]
        port: crate::port::PortArg,
    },
    /// Print resolved local bundle directories for configured knowledge roots.
    Roots {
        #[arg(long, value_enum, default_value_t = RootsFormatArg::Paths)]
        format: RootsFormatArg,
        /// Fetch git roots before printing, ignoring poll freshness.
        #[arg(long, overrides_with = "no_sync")]
        sync: bool,
        /// Print cache and directory paths without fetching.
        #[arg(long = "no-sync", overrides_with = "sync")]
        no_sync: bool,
    },
    /// Fetch configured git knowledge roots.
    Sync { id: Option<String> },
    /// Emit engine artifacts and the Askama HTML review tree.
    Build {
        #[arg(default_value = "knowledge")]
        root: PathBuf,
        #[arg(short, long, default_value = "dist/knowledge")]
        output: PathBuf,
        #[arg(long, value_enum, default_value_t = ProfileArg::Strict)]
        profile: ProfileArg,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum RootsFormatArg {
    Paths,
    Json,
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
enum TimingsScenarioArg {
    Load,
    Site,
    Click,
    Review,
    Log,
    Watch,
    #[default]
    All,
}

impl From<TimingsScenarioArg> for crate::timings::TimingsScenario {
    fn from(value: TimingsScenarioArg) -> Self {
        match value {
            TimingsScenarioArg::Load => Self::Load,
            TimingsScenarioArg::Site => Self::Site,
            TimingsScenarioArg::Click => Self::Click,
            TimingsScenarioArg::Review => Self::Review,
            TimingsScenarioArg::Log => Self::Log,
            TimingsScenarioArg::Watch => Self::Watch,
            TimingsScenarioArg::All => Self::All,
        }
    }
}

impl From<RootsFormatArg> for crate::roots::RootsFormat {
    fn from(value: RootsFormatArg) -> Self {
        match value {
            RootsFormatArg::Paths => Self::Paths,
            RootsFormatArg::Json => Self::Json,
        }
    }
}

#[derive(Args, Default)]
struct FiltersArg {
    /// Match any of these concept types. Repeat to add alternatives.
    #[arg(long = "type")]
    types: Vec<String>,
    /// Require this tag. Repeat to require multiple tags.
    #[arg(long = "tag")]
    tags: Vec<String>,
    /// Match any of these lifecycle statuses. Repeat to add alternatives.
    #[arg(long = "status")]
    statuses: Vec<String>,
    /// Match any of these authority levels. Repeat to add alternatives.
    #[arg(long = "authority")]
    authorities: Vec<String>,
    /// Match any of these derived trust tiers. Repeat to add alternatives.
    #[arg(long = "trust-tier", value_enum)]
    trust_tiers: Vec<TrustTierArg>,
    /// Match stale (`true`) or current (`false`) records.
    #[arg(long)]
    stale: Option<bool>,
}

impl From<&FiltersArg> for KnowledgeFilter {
    fn from(value: &FiltersArg) -> Self {
        Self {
            types: value.types.clone(),
            tags: value.tags.clone(),
            statuses: value.statuses.clone(),
            authorities: value.authorities.clone(),
            trust_tiers: value.trust_tiers.iter().copied().map(Into::into).collect(),
            stale: value.stale,
        }
    }
}

#[derive(Subcommand)]
enum InspectTarget {
    Catalog {
        #[arg(default_value = "knowledge")]
        root: PathBuf,
        #[command(flatten)]
        filters: FiltersArg,
    },
    Concept {
        concept: String,
        #[arg(default_value = "knowledge")]
        root: PathBuf,
    },
    Graph {
        #[arg(default_value = "knowledge")]
        root: PathBuf,
    },
}

pub fn run() -> Result<()> {
    let raw: Vec<String> = std::env::args().collect();
    let bundled = std::env::current_exe()
        .ok()
        .is_some_and(|exe| crate::bundle::running_inside_app_bundle(&exe));
    let cli = Cli::parse_from(crate::bundle::argv_for_parse(raw, bundled));
    match cli.command {
        Commands::Check {
            root,
            profile,
            format,
        } => {
            let report = check(&root, profile.into())?;
            print_check(&report, format)?;
            if report.has_errors() {
                bail!("knowledge check failed with errors");
            }
            Ok(())
        }
        Commands::Inspect { target, profile } => {
            let json = match target {
                InspectTarget::Catalog { root, filters } => inspect(
                    &root,
                    InspectKind::Catalog,
                    None,
                    profile.into(),
                    &(&filters).into(),
                )?,
                InspectTarget::Concept { concept, root } => inspect(
                    &root,
                    InspectKind::Concept,
                    Some(&concept),
                    profile.into(),
                    &KnowledgeFilter::default(),
                )?,
                InspectTarget::Graph { root } => inspect(
                    &root,
                    InspectKind::Graph,
                    None,
                    profile.into(),
                    &KnowledgeFilter::default(),
                )?,
            };
            println!("{json}");
            Ok(())
        }
        Commands::Search {
            query,
            root,
            profile,
            filters,
        } => {
            let json = search(&root, &query, profile.into(), &(&filters).into())?;
            println!("{json}");
            Ok(())
        }
        Commands::Timings {
            path,
            format,
            scenario,
            profile,
            provenance,
            no_provenance,
        } => crate::timings::run(crate::timings::TimingsOptions {
            path,
            format: match format {
                CheckFormat::Terminal => crate::timings::TimingsFormat::Terminal,
                CheckFormat::Json => crate::timings::TimingsFormat::Json,
            },
            scenario: scenario.into(),
            profile: profile.into(),
            provenance: if no_provenance {
                Some(false)
            } else if provenance {
                Some(true)
            } else {
                None
            },
        }),
        Commands::Benchmark {
            benchmark: path,
            root,
            profile,
        } => {
            let report = benchmark(&root, &path, profile.into())?;
            println!("{}", serde_json::to_string_pretty(&report)?);
            if !report.threshold_met {
                bail!(
                    "retrieval benchmark failed: hit rate {:.2}% was below required minimum {:.2}%",
                    report.hit_rate * 100.0,
                    report.minimum_hit_rate * 100.0
                );
            }
            Ok(())
        }
        Commands::Build {
            root,
            output,
            profile,
        } => {
            let summary = crate::site::build(&root, &output, profile.into())?;
            eprintln!(
                "okmate: built {} concepts and {} indexes into {}",
                summary.concepts, summary.indexes, summary.output
            );
            Ok(())
        }
        Commands::View {
            path,
            output,
            profile,
            no_window,
            public,
            port,
        } => crate::preview::run(crate::preview::ViewOptions {
            path,
            output,
            profile: profile.into(),
            public,
            port: port.resolve()?,
            no_window,
            allow_missing_bundle: bundled,
        }),
        Commands::Roots {
            format,
            sync,
            no_sync,
        } => {
            let mode = if no_sync {
                crate::roots::SyncMode::Never
            } else if sync {
                crate::roots::SyncMode::Force
            } else {
                crate::roots::SyncMode::Auto
            };
            crate::roots::print_roots(format.into(), mode)
        }
        Commands::Sync { id } => crate::roots::sync(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::PortArg;

    fn view_port(cli: &Cli) -> PortArg {
        match &cli.command {
            Commands::View { port, .. } => *port,
            _ => panic!("expected view"),
        }
    }

    #[test]
    fn clap_defaults_to_auto_with_window() {
        let cli = Cli::try_parse_from(["okmate", "view"]).unwrap();
        assert_eq!(view_port(&cli), PortArg::Auto);
    }

    #[test]
    fn clap_defaults_to_8000_without_window() {
        let cli = Cli::try_parse_from(["okmate", "view", "--no-window"]).unwrap();
        assert_eq!(view_port(&cli), PortArg::Exact(8000));
    }

    #[test]
    fn clap_accepts_port_auto() {
        let cli = Cli::try_parse_from(["okmate", "view", "--port", "auto"]).unwrap();
        assert_eq!(view_port(&cli), PortArg::Auto);
    }

    #[test]
    fn clap_accepts_explicit_port() {
        let cli = Cli::try_parse_from(["okmate", "view", "--port", "9001"]).unwrap();
        assert_eq!(view_port(&cli), PortArg::Exact(9001));
    }

    #[test]
    fn unpackaged_cli_requires_a_subcommand() {
        assert!(Cli::try_parse_from(["okmate"]).is_err());
    }

    #[test]
    fn version_matches_cargo_package() {
        let error = match Cli::try_parse_from(["okmate", "--version"]) {
            Err(error) => error,
            Ok(_) => panic!("expected --version to exit via clap"),
        };
        let rendered = error.to_string();
        assert!(rendered.contains(clap::crate_version!()), "{rendered}");
    }
}
