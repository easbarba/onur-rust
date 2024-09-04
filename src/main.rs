/*
* Onur is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* Onur is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with Onur. If not, see <https://www.gnu.org/licenses/>.
*/

use clap::{Parser, Subcommand};

mod actions;
mod commands;
mod domain;
mod misc;
mod storage;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Grab { config_topic }) => commands::grab::run(config_topic, true),
        Some(Commands::Backup { projects }) => commands::backup::run(projects),
        Some(Commands::Config {
            topic,
            name,
            url,
            branch,
        }) => commands::config::run(topic, name, url, branch),
        None => {}
    }
}

#[derive(Parser, Debug)]
#[command(name = "onur", version = "0.4.0", about = "Easily manage multiple FLOSS repositories.", long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "grab all")]
    Grab {
        #[arg()]
        config_topic: Option<String>,
    },

    #[command(about = "back up selected projects")]
    Backup {
        #[arg(required = true)]
        projects: Vec<String>,
    },

    #[command(about = "manage configuration")]
    Config {
        #[arg(required = true)]
        topic: String,

        #[arg(required = false, default_value = None)]
        name: Option<String>,

        #[arg(required = false, default_value = None)]
        url: Option<String>,

        #[arg(required = false, default_value = None)]
        branch: Option<String>,
    },
}
