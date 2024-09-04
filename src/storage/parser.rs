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

use std::collections::HashMap;
use std::path::Path;
use std::{fs, path};

use crate::domain::config::Config;
use crate::domain::project::Project;
use crate::storage::files;

// Parse single configuration
fn single(filepath: path::PathBuf) -> Config {
    let config_name = {
        match filepath.file_stem() {
            Some(v1) => match v1.to_str() {
                Some(v2) => v2,
                None => panic!("Unable to convert to string"),
            },
            None => panic!("Could not get file stem"),
        }
    };
    let file_content = config_read(&filepath);
    let projects: HashMap<String, Vec<Project>> = {
        match serde_json::from_str(&file_content) {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        }
    };

    Config::new(config_name.to_string(), projects)
}

// Collect multiple  parsed configurations
pub fn multi(verbose: bool) -> Vec<Config> {
    let mut configs: Vec<Config> = Vec::new();

    if verbose {
        print_info();
    }

    files::namepaths().unwrap().into_iter().for_each(|f| {
        configs.push(single(f.to_path_buf()));
    });

    configs
}

fn print_info() {
    println!("{} Configurations files found\n", files::count())
}

fn config_read(filepath: &Path) -> String {
    let readed = fs::read_to_string(filepath);

    match readed {
        Ok(content) => content,
        Err(err) => panic!("{}", err),
    }
}
