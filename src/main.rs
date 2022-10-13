use std::env::args_os;

use database::Database;

use crate::exit::Exit;

mod database;
mod exit;
mod file;
mod script;

fn main() {
    // Get the first arg.
    let first_arg = args_os().nth(1);

    // Convert it to `Option<&str>`.
    let param = first_arg.as_ref().and_then(|arg| arg.to_str());

    // Match `param`.
    match param {
        // If initialization flag is set, initialize a new run.yaml file.
        Some("-i" | "--init") => {
            // If package.json file exists in current directory, generate a script database using package.json scripts.
            if file::exists("package.json") {
                let package_json = file::read("package.json").exit();
                let db = Database::from_package_json(&package_json).exit();
                db.save().exit();
            }
            // If deno.json file exists in current directory, generate a script database using deno.json tasks.
            else if file::exists("deno.json") {
                let deno_json = file::read("deno.json").exit();
                let db = Database::from_deno_json(&deno_json).exit();
                db.save().exit();
            }
            // If no file above exists in current directory, generate a script database using example.
            else {
                let db = Database::from_example();
                db.save().exit();
            }
        }

        // If help flag is set, print a help message.
        Some("-h" | "--help") => {
            println!("Run is a tool to manage and execute your scripts.");
        }

        // If an alias or name is given, run the script associated with it.
        Some(alias_or_name) => {
            let run_yaml = file::read("run.yaml").exit();
            let db = Database::from_run_yaml(&run_yaml).exit();

            let exit_code = db.run(alias_or_name).exit();
            std::process::exit(exit_code);
        }

        // If no arg is given, print all the available scripts.
        None => {
            let run_yaml = file::read("run.yaml").exit();
            let database = Database::from_run_yaml(&run_yaml).exit();

            database.print();
        }
    }
}
