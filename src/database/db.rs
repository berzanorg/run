use std::collections::{BTreeMap, HashMap};

use crate::script::Script;

use super::errors::{FormatError, ParseError};

/// The type representing a B-Tree map for scripts.
pub type ScriptMap<'a> = BTreeMap<&'a str, Script<'a>>;

/// The type for an alias name.
pub enum AliasName<'a> {
    Name(&'a str),
    None,
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

    /// sda.
    pub(super) fn add(&mut self, name: &'a str, script: Script<'a>) -> Result<(), FormatError> {
        let alias = match name.chars().nth(0) {
            Some(ch) => ch,
            None => return Err(FormatError::NoName),
        };

        if script.command.is_empty() {
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
                    Some(AliasName::None) => (),
                    // If there is an `AliasName::Name(_)`, insert `AliasName::None`.
                    Some(AliasName::Name(_)) => {
                        self.alias_map.insert(alias, AliasName::None);
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
