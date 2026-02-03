use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(
    name = "guildsync",
    about = "Sync Discord guild dumps with terminal workflows; scaffold + spec",
    long_about = "A Rust CLI scaffold for synchronizing Discord guild dumps/upload formats with terminal workflows (OpenCode/Codex/tmux/interpreters/MCP).\n\nThis repository intentionally provides a coherent CLI surface + README specification, but does not implement real Discord/Kubernetes/SSH operations yet."
)]
struct Cli {
    /// Path to a config file (defaults to platform config location).
    #[arg(long)]
    config: Option<PathBuf>,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    json: bool,

    /// Logging verbosity.
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    log: LogLevel,

    #[command(subcommand)]
    command: Command,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Discord guild dump/export/import operations (stub).
    Discord {
        #[command(subcommand)]
        command: DiscordCommand,
    },

    /// Validate and convert between dump/upload formats (stub).
    Format {
        #[command(subcommand)]
        command: FormatCommand,
    },

    /// Integrations for terminal workflows (tmux, OpenCode/Codex) (stub).
    Terminal {
        #[command(subcommand)]
        command: TerminalCommand,
    },

    /// Kubernetes orchestration (local on-demand + remote test/deploy) (stub).
    Kube {
        #[command(subcommand)]
        command: KubeCommand,
    },

    /// SSH operations against remote computers (including over VPN) (stub).
    Ssh {
        #[command(subcommand)]
        command: SshCommand,
    },
}

#[derive(Subcommand, Debug)]
enum DiscordCommand {
    /// Export a guild to the guild dump format.
    Export {
        /// Discord guild ID.
        #[arg(long)]
        guild: u64,

        /// Output path for the dump JSON.
        #[arg(long)]
        out: PathBuf,
    },

    /// Import a dump/upload file into a guild.
    Import {
        /// Input file path.
        #[arg(long, value_name = "PATH")]
        r#in: PathBuf,

        /// Discord guild ID.
        #[arg(long)]
        guild: u64,

        /// Only validate inputs and show planned actions.
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand, Debug)]
enum FormatCommand {
    /// Validate a dump or upload-format file.
    Validate {
        /// Input file path.
        #[arg(long, value_name = "PATH")]
        r#in: PathBuf,

        /// Expected format.
        #[arg(long, value_enum)]
        format: Option<GuildFormat>,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum GuildFormat {
    Dump,
    Upload,
}

#[derive(Subcommand, Debug)]
enum TerminalCommand {
    /// Attach the current workflow to an OpenCode/Codex session (documentation only).
    Opencode {
        #[command(subcommand)]
        command: TerminalOpenCodeCommand,
    },
}

#[derive(Subcommand, Debug)]
enum TerminalOpenCodeCommand {
    /// Attach to a tmux session intended to host OpenCode/Codex and interpreters.
    Attach {
        /// tmux session name.
        #[arg(long)]
        tmux: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum KubeCommand {
    /// Local on-demand cluster workflows (kind/k3d/minikube) (stub).
    Local {
        #[command(subcommand)]
        command: KubeLocalCommand,
    },

    /// Remote cluster workflows (test/deploy) by kube context (stub).
    Remote {
        #[command(subcommand)]
        command: KubeRemoteCommand,
    },
}

#[derive(Subcommand, Debug)]
enum KubeLocalCommand {
    Up,
    Down,
    Status,
}

#[derive(Subcommand, Debug)]
enum KubeRemoteCommand {
    /// Run on-demand tests against a remote cluster.
    Test {
        /// kubeconfig context name.
        #[arg(long)]
        context: String,
    },

    /// Deploy to a remote cluster.
    Deploy {
        /// kubeconfig context name.
        #[arg(long)]
        context: String,
    },
}

#[derive(Subcommand, Debug)]
enum SshCommand {
    /// Execute a command on a remote host.
    Exec {
        /// Host (SSH config host alias or hostname).
        #[arg(long)]
        host: String,

        /// Command to execute remotely.
        #[arg(last = true, required = true)]
        cmd: Vec<String>,
    },
}

#[derive(Serialize)]
struct JsonOut<'a> {
    ok: bool,
    action: &'a str,
    message: &'a str,
}

#[derive(Debug, Error)]
enum CliError {
    #[error("scaffold only; not implemented")]
    NotImplemented,
    #[error("failed to read input file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("input must be a JSON object")]
    NotObject,
    #[error("format mismatch: expected {expected}, found {found}")]
    FormatMismatch { expected: String, found: String },
    #[error("unrecognized format value: {0}")]
    UnknownFormat(String),
    #[error("format must be a string when provided")]
    FormatNotString,
    #[error("version must be an unsigned integer when provided")]
    InvalidVersionType,
}

struct ValidationSummary {
    format: Option<String>,
    version: Option<u64>,
    format_declared: bool,
}

impl GuildFormat {
    fn as_str(&self) -> &'static str {
        match self {
            GuildFormat::Dump => "dump",
            GuildFormat::Upload => "upload",
        }
    }
}

fn action_for(command: &Command) -> &'static str {
    match command {
        Command::Discord { command } => match command {
            DiscordCommand::Export { .. } => "discord.export",
            DiscordCommand::Import { .. } => "discord.import",
        },
        Command::Format { command } => match command {
            FormatCommand::Validate { .. } => "format.validate",
        },
        Command::Terminal { command } => match command {
            TerminalCommand::Opencode { command } => match command {
                TerminalOpenCodeCommand::Attach { .. } => "terminal.opencode.attach",
            },
        },
        Command::Kube { command } => match command {
            KubeCommand::Local { command } => match command {
                KubeLocalCommand::Up => "kube.local.up",
                KubeLocalCommand::Down => "kube.local.down",
                KubeLocalCommand::Status => "kube.local.status",
            },
            KubeCommand::Remote { command } => match command {
                KubeRemoteCommand::Test { .. } => "kube.remote.test",
                KubeRemoteCommand::Deploy { .. } => "kube.remote.deploy",
            },
        },
        Command::Ssh { command } => match command {
            SshCommand::Exec { .. } => "ssh.exec",
        },
    }
}

fn validate_format(path: &PathBuf, expected: Option<GuildFormat>) -> Result<ValidationSummary, CliError> {
    let contents = std::fs::read_to_string(path)?;
    let value: serde_json::Value = serde_json::from_str(&contents)?;
    let object = value.as_object().ok_or(CliError::NotObject)?;

    let mut format = None;
    let mut format_declared = false;
    if let Some(format_value) = object.get("format") {
        format_declared = true;
        let format_str = format_value.as_str().ok_or(CliError::FormatNotString)?;
        match format_str {
            "dump" | "upload" => {
                format = Some(format_str.to_string());
            }
            other => return Err(CliError::UnknownFormat(other.to_string())),
        }
    }

    if let Some(expected_format) = expected {
        if let Some(found) = format.as_deref() {
            if found != expected_format.as_str() {
                return Err(CliError::FormatMismatch {
                    expected: expected_format.as_str().to_string(),
                    found: found.to_string(),
                });
            }
        }
    }

    let mut version = None;
    if let Some(version_value) = object.get("version") {
        let version_num = version_value.as_u64().ok_or(CliError::InvalidVersionType)?;
        version = Some(version_num);
    }

    Ok(ValidationSummary {
        format,
        version,
        format_declared,
    })
}

fn main() {
    let cli = Cli::parse();
    let action = action_for(&cli.command);

    let result = match &cli.command {
        Command::Format { command } => match command {
            FormatCommand::Validate { r#in, format } => {
                validate_format(r#in, *format).map(|summary| {
                    let mut details = Vec::new();
                    if let Some(expected) = format {
                        details.push(format!("expected={}", expected.as_str()));
                        if !summary.format_declared {
                            details.push("format=missing".to_string());
                        }
                    }
                    if let Some(found) = summary.format {
                        details.push(format!("format={found}"));
                    }
                    if let Some(version) = summary.version {
                        details.push(format!("version={version}"));
                    }
                    if details.is_empty() {
                        "valid JSON input".to_string()
                    } else {
                        format!("valid JSON input ({})", details.join(", "))
                    }
                })
            }
        },
        _ => Err(CliError::NotImplemented),
    };

    match result {
        Ok(message) => {
            if cli.json {
                let out = JsonOut {
                    ok: true,
                    action,
                    message: &message,
                };
                println!(
                    "{}",
                    serde_json::to_string_pretty(&out)
                        .unwrap_or_else(|_| "{\"ok\":true}".to_string())
                );
            } else {
                println!("{action}: {message}");
            }
        }
        Err(err) => {
            let exit_code = match err {
                CliError::NotImplemented => 2,
                _ => 1,
            };
            if cli.json {
                let out = JsonOut {
                    ok: false,
                    action,
                    message: &err.to_string(),
                };
                println!(
                    "{}",
                    serde_json::to_string_pretty(&out)
                        .unwrap_or_else(|_| "{\"ok\":false}".to_string())
                );
            } else {
                eprintln!("{action}: {err}");
                if matches!(err, CliError::NotImplemented) {
                    eprintln!("config: {:?}", cli.config);
                    eprintln!("log: {:?}", cli.log);
                }
            }
            std::process::exit(exit_code);
        }
    }
}
