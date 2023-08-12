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

use std::{path::PathBuf, process::exit};

use git2::Repository;

#[path = "../osm/parser.rs"]
mod parser;

#[path = "../misc/globals.rs"]
mod globals;

fn clone(file: &serde_json::Value, path: PathBuf) {
    match Repository::clone(
        file["url"].as_str().expect("There should be a url here"),
        path,
    ) {
        Ok(repo) => repo,
        Err(_) => {
            println!("\nError: Repository already cloned!");
            exit(1);
        }
    };
}

fn pull(file: &serde_json::Value, path: PathBuf) {
    print!("PULLING: {}, {}", file.as_str().unwrap(), path.display())
}

pub fn run(verbose: bool) {
    let file = &parser::all(verbose)[0];
    let path = globals::projects_home().join("gitignore");

    print_info(file, verbose);

    if Repository::open(path.clone()).is_ok() {
        pull(file, path);
    } else {
        clone(file, path);
    }
}

fn print_info(file: &serde_json::Value, verbose: bool) {
    print!("name: {}", file["name"]);

    if verbose {
        print!("branch: {}, url: {}", file["branch"], file["url"]);
    }
}

// fn fast_forward(&self, path: &Path) -> Result<(), Error> {
//     let repo = Repository::open(path)?;

//     repo.find_remote("origin")?
//         .fetch(&[self.branch], None, None)?;

//     let fetch_head = repo.find_reference("FETCH_HEAD")?;
//     let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
//     let analysis = repo.merge_analysis(&[&fetch_commit])?;
//     if analysis.0.is_up_to_date() {
//         Ok(())
//     } else if analysis.0.is_fast_forward() {
//         let refname = format!("refs/heads/{}", self.branch);
//         let mut reference = repo.find_reference(&refname)?;
//         reference.set_target(fetch_commit.id(), "Fast-Forward")?;
//         repo.set_head(&refname)?;
//         repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
//     } else {
//         Err(Error::from_str("Fast-forward only!"))
//     }
// }

// pub fn check(&self) {
//     if repo_path.exists() && repo_path.is_dir() {
//         self.reset(repo_path);
//         if let Err(e) = self.fast_forward(repo_path) {
//             panic!("Failed to pull: {}", e)
//         }
//     }
// }
