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

use std::path::PathBuf;

#[path = "../misc/globals.rs"]
mod globals;

// do config folder exist?
pub fn exist() -> bool {
    globals::config_home().exists()
}

// is config folder empty?
pub fn empty() -> bool {
    globals::config_home()
        .read_dir()
        .expect("meh")
        .next()
        .is_none()
}

pub fn count() -> usize {
    globals::config_home().read_dir().expect("foool").count()
}

// list all configurations file names
pub fn names() -> Result<Vec<PathBuf>, std::io::Error> {
    if !exist() || empty() {
        print!("No configuration file found, exiting!");
        std::process::exit(1);
    }

    return Ok(globals::config_home()
        .read_dir()?
        .filter_map(|res| res.ok())
        .map(|d| d.path())
        .filter(|x| x.to_string_lossy().ends_with(".json"))
        .collect::<Vec<_>>());
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
        assert_eq!(count(), 5);
    }
}
