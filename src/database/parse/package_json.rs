use crate::{
    database::{db::Database, errors::ParseError},
    script::Script,
};

const PACKAGE_JSON: &'static str = "package.json";

impl<'a> Database<'a> {
    pub fn from_package_json(package_json: &'a str) -> Result<Database<'a>, ParseError> {
        let mut db = Database::new();
        let chars = package_json.chars().collect::<Vec<char>>();

        let mut bufs: Vec<(&'a str, usize)> = vec![];

        let mut line_no = 1;

        let mut is_inside_quotes = false;
        let mut is_inside_scripts = false;

        let mut start_index = 0;
        let mut all_len = 0;

        for i in 0..chars.len() {
            let ch = chars[i];
            let prev_ch = if i > 0 { chars.get(i - 1) } else { None };

            match ch {
                '"' => {
                    if prev_ch != Some(&'\\') {
                        is_inside_quotes = !is_inside_quotes;
                        if is_inside_quotes {
                            start_index = all_len + 1;
                        } else {
                            let buf = &package_json[start_index..all_len];

                            if is_inside_scripts {
                                bufs.push((buf, line_no));
                            }

                            if buf == "scripts" {
                                is_inside_scripts = true;
                            }
                            start_index = 0;
                        }
                    }
                }

                '}' => {
                    if !is_inside_quotes && is_inside_scripts {
                        break;
                    }
                }

                '\n' => {
                    line_no += 1;
                }

                _ => (),
            }

            all_len += ch.len_utf8();
        }

        for i in 0..bufs.len() {
            if i % 2 == 0 {
                let (mut name, name_line_no) = bufs[i];

                name = name.trim();

                let command = match bufs.get(i + 1) {
                    Some((command, _)) => command.trim(),
                    None => return Err(ParseError::NoCommand(name_line_no, PACKAGE_JSON)),
                };

                match db.add(name, Script::new(command)) {
                    Ok(()) => (),
                    Err(err) => return Err(err.into_parse_error(name_line_no, PACKAGE_JSON)),
                };
            }
        }

        Ok(db)
    }
}

#[test]
fn test() {
    // Create a test content.
    let package_json = r#"{
        "name": "wrapn",
        "version": "1.0.0",
        "scripts": {
            "compile": "tsc",
            "bundle": "rollup -c"
        },
        "peerDependencies": {
            "react": "^18.2.0"
        }"#;

    // Generate a database from package.json content.
    let db = Database::from_package_json(package_json).ok().unwrap();

    // Get all the scripts and names from the database.
    let scripts_and_names = db.scripts_and_names();

    // There must be 2 scripts.
    assert_eq!(scripts_and_names.len(), 2);

    // Get first script.
    let (bundle_name, bundle_script) = scripts_and_names.get(0).unwrap();

    // Get second script.
    let (compile_name, compile_script) = scripts_and_names.get(1).unwrap();

    // Check names.
    assert_eq!(bundle_name, &&"bundle");
    assert_eq!(compile_name, &&"compile");

    // Check commands.
    assert_eq!(bundle_script.command(), "rollup -c");
    assert_eq!(compile_script.command(), "tsc");

    // As package.json scripts doesn't have comments, we don't have to check them.
}
