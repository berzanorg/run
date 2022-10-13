use crate::{
    database::{db::Database, errors::ParseError},
    script::Script,
};

pub const RUN_YAML: &'static str = "run.yaml";

impl<'a> Database<'a> {
    pub fn from_run_yaml(run_yaml: &'a str) -> Result<Database<'a>, ParseError> {
        // Create a new database.
        let mut db = Database::new();

        // Define a variable to hold the last comment inside run.yaml.
        let mut last_comment: Option<&'a str> = None;

        // Get each line and line index.
        for (line_index, line) in run_yaml.lines().enumerate() {
            // Remove leading and trailing whitespaces.
            let line = line.trim();

            // Only continue if line is not empty.
            if !line.is_empty() {
                // If a line start with sharp symbol, that means it is a comment.
                if line.starts_with("#") {
                    // Set `last_comment` if there isn't `Some(comment)` inside.
                    if last_comment.is_none() {
                        last_comment = Some(&line[1..].trim());
                    }
                    // There is an unexpected comment.
                    else {
                        return Err(ParseError::UnexpectedComment(line_index + 1, RUN_YAML));
                    }
                }
                // If a line isn't a comment, it must hold a script name and script command.
                else {
                    // Try to split the line.
                    match line.split_once(": ") {
                        // If the splition is succesfull, continue.
                        Some((key, value)) => {
                            // Remove leading and trailing whitespaces.
                            let name = key.trim();
                            let command = value.trim();

                            // Create a new `Script`.
                            let script = match last_comment {
                                Some(comment) => Script::new_with_comment(command, comment),
                                None => Script::new(command),
                            };

                            // It has been used, so set it to `None`.
                            last_comment = None;

                            // Try to add name and script into database.
                            match db.add(name, script) {
                                Ok(()) => (),
                                Err(err) => {
                                    return Err(err.into_parse_error(line_index + 1, RUN_YAML))
                                }
                            }
                        }
                        // If the splition is failed, return an error.
                        None => return Err(ParseError::NoColon(line_index + 1, RUN_YAML)),
                    }
                }
            }
        }

        // If current run.yaml content is bad, format it and save.
        db.save_if_bad(run_yaml);

        // Successfully return the database.
        Ok(db)
    }
}

#[test]
fn test() {
    // Create a test content.
    let run_yaml = r#"
        # Prints a greeting message.
        greet: echo hey!

        # Compiles the project.
        compile: tsc

        "#;

    // Generate a database from run.yaml content.
    let db = Database::from_run_yaml(run_yaml).ok().unwrap();

    // Get all the scripts and names from the database.
    let scripts_and_names = db.scripts_and_names();

    // There must be 2 scripts.
    assert_eq!(scripts_and_names.len(), 2);

    // Get first script.
    let (compile_name, compile_script) = scripts_and_names.get(0).unwrap();

    // Get second script.
    let (greet_name, greet_script) = scripts_and_names.get(1).unwrap();

    // Check names.
    assert_eq!(compile_name, &&"compile");
    assert_eq!(greet_name, &&"greet");

    // Check commands.
    assert_eq!(compile_script.command(), "tsc");
    assert_eq!(greet_script.command(), "echo hey!");

    // Check comments.
    assert_eq!(compile_script.comment(), "Compiles the project.");
    assert_eq!(greet_script.comment(), "Prints a greeting message.");
}
