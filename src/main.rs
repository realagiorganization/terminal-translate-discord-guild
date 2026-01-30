use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

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

fn main() {
    let cli = Cli::parse();

    // Note: this is a scaffold. All commands intentionally return "not implemented".
    let action = match &cli.command {
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
    };

    if cli.json {
        let out = JsonOut {
            ok: false,
            action,
            message: "scaffold only; not implemented",
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&out).unwrap_or_else(|_| "{\"ok\":false}".to_string())
        );
        return;
    }

    eprintln!("{action}: scaffold only; not implemented");
    eprintln!("config: {:?}", cli.config);
    eprintln!("log: {:?}", cli.log);
    std::process::exit(2);
}
