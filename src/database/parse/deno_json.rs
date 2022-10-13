use crate::database::{db::Database, errors::ParseError};

const DENO_JSON: &'static str = "deno.json";

impl<'a> Database<'a> {
    /// Creates a database by searching for script names and commands inside `"tasks"` object inside `deno_json`. 
    pub fn from_deno_json(deno_json: &'a str) -> Result<Database<'a>, ParseError> {
        Database::from_json_object(deno_json, "tasks", DENO_JSON)
    }
}

#[test]
fn test() {
    // Create a test content.
    let deno_json = r#"{
        "tasks": {
          "start": "deno run -A --watch=static/,routes/ dev.ts"
        },
        "importMap": "./import_map.json",
        "compilerOptions": {
          "jsx": "react-jsx",
          "jsxImportSource": "preact"
        }
      }"#;

    // Generate a database from deno.json content.
    let db = Database::from_deno_json(deno_json).ok().unwrap();

    // Get all the scripts and names from the database.
    let scripts_and_names = db.scripts_and_names();

    // There must be 1 script.
    assert_eq!(scripts_and_names.len(), 1);

    // Get first script.
    let (start_name, start_script) = scripts_and_names.get(0).unwrap();

    // Check names.
    assert_eq!(start_name, &&"start");

    // Check commands.
    assert_eq!(
        start_script.command(),
        "deno run -A --watch=static/,routes/ dev.ts"
    );

    // As deno.json scripts doesn't have comments, we don't have to check them.
}
