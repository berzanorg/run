use crate::{
    database::{db::Database, errors::ParseError},
    script::Script,
};

const RUN_YAML: &'static str = "run.yaml";

impl<'a> Database<'a> {
    pub fn from_run_yaml(run_yaml: &'a str) -> Result<Database<'a>, ParseError> {
        let mut db = Database::new();
        let mut last_comment: Option<&'a str> = None;

        for (line_index, line) in run_yaml.lines().enumerate() {
            let line = line.trim();

            if !line.is_empty() {
                if line.starts_with("#") {
                    if last_comment.is_none() {
                        last_comment = Some(&line[1..].trim());
                    } else {
                        return Err(ParseError::UnexpectedComment(line_index + 1, RUN_YAML));
                    }
                } else {
                    match line.split_once(": ") {
                        Some((key, value)) => {
                            let name = key.trim();
                            let script = Script::new_with_comment(
                                value.trim(),
                                last_comment
                                    .and_then(|comment| {
                                        Some(if comment.is_empty() {
                                            "No comment specified."
                                        } else {
                                            comment
                                        })
                                    })
                                    .unwrap_or("No comment specified."),
                            );
                            last_comment = None;

                            match db.add(name, script) {
                                Ok(()) => (),
                                Err(err) => {
                                    return Err(err.into_parse_error(line_index + 1, RUN_YAML))
                                }
                            }
                        }
                        None => return Err(ParseError::NoColon(line_index + 1, RUN_YAML)),
                    }
                }
            }
        }

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
