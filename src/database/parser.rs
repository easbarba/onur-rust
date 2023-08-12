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

use std::{collections::HashMap, fs};

#[path = "./files.rs"]
mod files;

#[path = "../domain/project.rs"]
mod project;

// Parse one configuration
fn one(f: std::path::PathBuf) -> Vec<project::Project> {
    let file = fs::read_to_string(f.as_path()).expect("gimme it");
    let result: Vec<project::Project> = serde_json::from_str(&file).expect("translate it");

    result
}

// Collect all parsed configurations
pub fn all(verbose: bool) -> HashMap<String, Vec<project::Project>> {
    let mut configs: HashMap<String, Vec<project::Project>> = HashMap::new();

    if verbose {
        print_info();
    }

    files::names().unwrap().into_iter().for_each(|f| {
        configs.insert(f.file_stem().unwrap().to_str().unwrap().to_string(), one(f));
    });

    return configs;
}

pub fn print_info() {
    println!("{} Configurations files found\n", files::count())
}
