/*
 * Copyright 2023 EAS Barbosa
 *
 *     Licensed under the Apache License, Version 2.0(the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{fs, path};

use crate::database::files;
use crate::domain::config::Config;
use crate::domain::project::Project;

// Parse one configuration
fn one(filepath: path::PathBuf) -> Config {
    let file = fs::read_to_string(filepath.as_path()).expect("gimme it");
    let topic = filepath.file_stem().unwrap().to_str().unwrap().to_string();
    let projects: Vec<Project> = serde_json::from_str(&file).expect("translate it");

    Config::new(topic, projects)
}

// Collect all parsed configurations
pub fn all(verbose: bool) -> Vec<Config> {
    let mut configs: Vec<Config> = Vec::new();

    if verbose {
        print_info();
    }

    crate::database::files::names()
        .unwrap()
        .into_iter()
        .for_each(|f| {
            configs.push(one(f.to_path_buf()));
        });

    return configs;
}

pub fn print_info() {
    println!("{} Configurations files found\n", files::count())
}
