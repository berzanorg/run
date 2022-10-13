use crate::database::{db::Database, errors::ParseError};

const PACKAGE_JSON: &'static str = "package.json";

impl<'a> Database<'a> {
    /// Creates a database by searching for script names and commands inside `"scripts"` object inside `package_json`. 
    pub fn from_package_json(package_json: &'a str) -> Result<Database<'a>, ParseError> {
        Database::from_json_object(package_json, "scripts", PACKAGE_JSON)
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
