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

use super::project::Project;

pub struct Config {
    pub name: String,
    pub topics: HashMap<String, Vec<Project>>,
}

impl Config {
    pub fn new(name: String, topics: HashMap<String, Vec<Project>>) -> Config {
        Config { name, topics }
    }
}
