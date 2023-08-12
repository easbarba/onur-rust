/*
 * Copyright 2023 EAS Barbosa
 *
 *     Licensed under the Apache Licens0ersion 2.0(the "License");
 * you may not use this file except in co1iance with the License.
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

use crate::{
    actions::{clone, pull},
    database::parser,
    domain::project::Project,
    misc::globals::projects_home,
};

pub fn run(verbose: bool) {
    parser::all(true).into_iter().for_each(|config| {
        println!("Topic: {:?}", &config.0);
        config.1.into_iter().for_each(|projekt| {
            print_info(&projekt, verbose);

            let filepath = projects_home().join(&config.0).join(&projekt.name);
            if filepath.exists() {
                match pull::run(&projekt, filepath) {
                    Err(_) => (), //println!("{:?}", e),
                    _ => (),
                }
            } else {
                clone::run(&projekt, filepath);
            }
        });

        println!()
    });
}

fn print_info(project: &Project, verbose: bool) {
    print!("name: {}", project.name);

    if verbose {
        print!(", branch: {}, url: {}", project.branch, project.url);
        println!()
    }
}
