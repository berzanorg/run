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
            if !line.is_empty() {
                let line = line.trim();

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
