pub mod install;
pub mod list;
pub mod remove;
pub mod sync;
pub mod upgrade;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    #[arg(long, short)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Install(install::Args),
    Remove(remove::Args),
    Sync(sync::Args),
    Upgrade(upgrade::Args),
    List(list::Args),
    Complete(complete::Args),
}

pub mod complete {
    use anyhow::Result;
    use clap::ValueEnum;
    use clap::{CommandFactory, Parser};
    use clap_complete::{generate, shells};
    use std::io;

    #[derive(Parser, Debug, Clone)]
    pub struct Args {
        #[arg(long)]
        shell: Shell,
    }

    #[derive(Clone, Debug, ValueEnum)]
    enum Shell {
        Bash,
        Zsh,
        Elvish,
        Fish,
        #[allow(clippy::enum_variant_names)]
        PowerShell,
        Nushell,
    }

    impl clap_complete::Generator for Shell {
        fn generate(&self, cmd: &clap::Command, buf: &mut dyn io::Write) {
            match *self {
                Self::Bash => shells::Shell::Bash.generate(cmd, buf),
                Self::Zsh => shells::Shell::Zsh.generate(cmd, buf),
                Self::Elvish => shells::Shell::Elvish.generate(cmd, buf),
                Self::Fish => shells::Shell::Fish.generate(cmd, buf),
                Self::PowerShell => shells::PowerShell.generate(cmd, buf),
                Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            }
        }

        fn file_name(&self, name: &str) -> String {
            match *self {
                Self::Bash => shells::Shell::Bash.file_name(name),
                Self::Zsh => shells::Shell::Zsh.file_name(name),
                Self::Elvish => shells::Shell::Elvish.file_name(name),
                Self::Fish => shells::Shell::Fish.file_name(name),
                Self::PowerShell => shells::PowerShell.file_name(name),
                Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            }
        }
    }

    pub fn cmd(args: Args) -> Result<()> {
        let mut cmd = super::Args::command();
        let bin_name = cmd.get_name().to_string();
        generate(args.shell, &mut cmd, bin_name, &mut io::stdout());
        Ok(())
    }
}
