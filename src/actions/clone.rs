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

use std::path::PathBuf;

use crate::domain::project::Project;

pub fn run_raw(project: &Project, project_path: PathBuf) {
    if let Some(ppath) = project_path.to_str() {
        let args = vec![
            "clone",
            "--single-branch",
            "--depth=1",
            "--quiet",
            &project.url,
            ppath,
        ];

        if let Err(e) = std::process::Command::new("git").args(args).status() {
            eprintln!("{}", e);
        };
    };
}

// use git2::build;

// pub fn run(project: &crate::domain::project::Project, path: PathBuf, _verbose: bool) {
//     let mut bu = build::RepoBuilder::new();
//     bu.branch(&project.branch);

//     match bu.clone(&project.url, &path) {
//         Ok(repo) => repo,
//         Err(e) => panic!("failed to clone: {}", e),
//     };
// }
