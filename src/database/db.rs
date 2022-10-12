use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

use crate::{
    file::{self, FileError},
    script::Script,
};

use super::errors::{DatabaseError, FormatError};

/// The type representing a B-Tree map for scripts.
pub type ScriptMap<'a> = BTreeMap<&'a str, Script<'a>>;

/// The type for an alias name.
pub enum AliasName<'a> {
    Name(&'a str),
    Multi,
}

/// The type representing a hash map for aliasses.
pub type AliasMap<'a> = HashMap<char, AliasName<'a>>;

/// A database stores scripts, their names, and aliasses.
pub struct Database<'a> {
    script_map: ScriptMap<'a>,
    alias_map: AliasMap<'a>,
}

impl<'a> Database<'a> {
    /// Creates a new `Database`.
    pub(super) fn new() -> Database<'a> {
        Database {
            script_map: ScriptMap::new(),
            alias_map: AliasMap::new(),
        }
    }

    /// Prints all the available scripts' names and comments.
    pub fn print(&self) {
        let mut content = "Scripts:\n".to_string();

        for (name, script) in &self.script_map {
            content = content + "    " + name + "   " + script.comment() + "\n\n";
        }

        println!("{}", content);
    }

    /// Extracts the entire database into a `String`, to be saved later.
    pub(super) fn extract(&self) -> String {
        let mut extraction = String::new();

        for (name, script) in &self.script_map {
            extraction = format!(
                "{}# {}\n{}: {}\n\n",
                extraction,
                script.comment(),
                name,
                script.command()
            );
        }

        extraction
    }

    #[cfg(test)]
    /// Returns a vector of tuples containing scripts and their names.
    pub(super) fn scripts_and_names(&self) -> Vec<(&&'a str, &'a Script)> {
        self.script_map.iter().collect()
    }

    /// Saves the database to run.yaml file in the current directory.
    pub fn save(&self) -> Result<(), FileError> {
        file::create("run.yaml", &self.extract())
    }

    /// Saves the database to run.yaml file in the current directory.
    pub fn save_if_bad(&self, run_yaml: &'a str) {
        let extraction = self.extract();

        if extraction != run_yaml {
            // We don't need to be warned, cuz the result won't affect any operation.
            #[allow(unused_must_use)]
            {
                file::write("run.yaml", &extraction);
            }
        };
    }

    /// Executes the associated script, and then returns it's exit code.
    pub fn run(&self, alias_or_name: &'a str) -> Result<i32, DatabaseError> {
        let (name, script) = self.get(alias_or_name)?;

        println!("run {}", name);

        let start_time = Instant::now();

        let exit_code = if file::exists("package.json") {
            script.execute(Some("node_modules/.bin"))
        } else {
            script.execute(None)
        };

        let end_time = start_time.elapsed();

        match exit_code {
            0 => println!("in {:.2?}", end_time),
            _ => println!("{}", exit_code),
        }

        Ok(exit_code)
    }

    /// Returns a tuple of associated `Script` and its name.
    fn get(&self, alias_or_name: &'a str) -> Result<(&'a str, &'a Script), DatabaseError<'a>> {
        let chars: Vec<char> = alias_or_name.chars().collect();

        if chars.len() == 1 {
            let alias = chars[0];

            match self.alias_map.get(&alias) {
                Some(alias_name) => match alias_name {
                    AliasName::Name(name) => match self.script_map.get(name) {
                        Some(script) => Ok((name, script)),
                        None => Err(DatabaseError::NoName(name)),
                    },

                    AliasName::Multi => Err(DatabaseError::MultiAlias(alias)),
                },

                None => Err(DatabaseError::NoAlias(alias)),
            }
        } else {
            let name = alias_or_name;

            match self.script_map.get(name) {
                Some(script) => Ok((name, script)),
                None => Err(DatabaseError::NoName(name)),
            }
        }
    }

    /// Adds a new script and its name to `self.script_map`.
    ///
    /// Also creates an alias for the name, if possible.
    pub(super) fn add(&mut self, name: &'a str, script: Script<'a>) -> Result<(), FormatError> {
        let alias = match name.chars().nth(0) {
            Some(ch) => ch,
            None => return Err(FormatError::NoName),
        };

        if script.command().is_empty() {
            return Err(FormatError::NoCommand);
        }

        if name.starts_with("-") {
            return Err(FormatError::MinusInStartOfName);
        }

        if name.contains(" ") {
            return Err(FormatError::SpaceInName);
        }

        // If the same key is used before, return `UsedName` error.
        match self.script_map.insert(name, script) {
            Some(_) => Err(FormatError::UsedName),
            None => {
                // Try to get the value of given key.
                match self.alias_map.get(&alias) {
                    // If there is an `AliasName::None`, do nothing.
                    Some(AliasName::Multi) => (),
                    // If there is an `AliasName::Name(_)`, insert `AliasName::None`.
                    Some(AliasName::Name(_)) => {
                        self.alias_map.insert(alias, AliasName::Multi);
                    }
                    // If the same key is never user, insert `AliasName::Name(name)`.
                    None => {
                        self.alias_map.insert(alias, AliasName::Name(name));
                    }
                }
                Ok(())
            }
        }
    }
}
