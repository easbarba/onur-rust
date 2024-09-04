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

use crate::{
    actions::{clone, pull},
    domain::project::Project,
    misc::globals::projects_home,
    storage::parser,
};

pub fn run(config_topic: &Option<String>, _verbose: bool) {
    println!(
        "Config/Topic: {}",
        if config_topic.is_some() {
            config_topic.as_ref().unwrap().to_string()
        } else {
            "none".to_string()
        }
    );

    parser::multi(true).into_iter().for_each(|config| {
        println!("{}:", &config.name.to_string());

        config.topics.into_iter().for_each(|topic| {
            println!("{:<2}+ {}", "", topic.0);

            topic.1.into_iter().for_each(|project| {
                print_project_info(&project);

                let project_path = projects_home()
                    .join(&config.name)
                    .join(&topic.0)
                    .join(&project.name);

                // if project_path.to_str().is_some() {
                //     println!("{}", project_path.to_str().unwrap());
                // }

                if project_path.exists() {
                    pull::run_raw(project_path);
                } else {
                    clone::run_raw(&project, project_path);
                }
            });
        });

        println!();
    });
}

fn print_project_info(project: &Project) {
    println!(
        "{:<4}- {:<35} {:<75} {}",
        "",
        if project.name.len() > 35 {
            format!("{}...", &project.name[..30])
        } else {
            project.name.to_string()
        },
        if project.url.len() > 75 {
            format!("{}...", &project.url[..70])
        } else {
            project.url.to_string()
        },
        &project.branch
    );
}
