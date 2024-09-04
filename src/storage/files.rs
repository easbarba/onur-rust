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

use crate::misc::globals;

// list all configurations file names
pub fn namepaths() -> Result<Vec<PathBuf>, std::io::Error> {
    if !exist() || empty() {
        print!("No configuration file found, exiting!");
        std::process::exit(1);
    }

    Ok(all()
        .filter_map(|res| res.ok())
        .map(|d| d.path())
        .filter(|x| x.to_string_lossy().ends_with(".json"))
        .filter(|f| f.exists())
        .collect::<Vec<PathBuf>>())
}

pub fn count() -> usize {
    if !exist() {
        return 0;
    }

    all().count()
}

// is config folder empty?
fn empty() -> bool {
    if !exist() {
        return true;
    }

    all().next().is_none()
}

// do config folder exist?
fn exist() -> bool {
    globals::config_home().exists()
}

fn all() -> std::fs::ReadDir {
    match globals::config_home().read_dir() {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_home_test() {
        assert_eq!(
            PathBuf::from(globals::user_home().join(".config/onur")),
            globals::config_home()
        );
    }
    #[test]
    fn config_folder_exist() {
        assert!(exist());
    }

    #[test]
    fn config_folder_is_empty() {
        assert!(!empty());
    }

    #[test]
    fn config_folder_count_correctly() {
        assert_eq!(count(), 8);
    }
}
