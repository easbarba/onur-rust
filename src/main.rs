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

#[path = "./misc/cli.rs"]
mod cli;

#[path = "./database/parser.rs"]
mod parser;

fn main() {
    let matches = cli::options().get_matches();
    match matches.subcommand() {
        Some(("grab", _)) => parser::all(true).into_iter().for_each(|config| {
            println!("Topic: {:?}", config.0);
            config
                .1
                .into_iter()
                .for_each(|project| println!("{:?}", project));

            println!();
        }),
        Some(("backup", sub_matches)) => {
            println!(
                "backing up {:?}",
                sub_matches
                    .get_one::<String>("resources")
                    .expect("required")
                    .split(",")
                    .collect::<Vec<&str>>()
            );
        }
        _ => unreachable!(),
    }
}
