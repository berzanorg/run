use std::collections::BTreeMap;

/// The type representing `&'a str`.
pub type Str<'a> = &'a str;

/// The type representing script names.
pub type ScriptName<'a> = Str<'a>;

/// The type representing script codes.
type ScriptCode<'a> = Str<'a>;

/// The type representing script comments.
pub type ScriptComment<'a> = Str<'a>;

/// The type representing scripts.
pub type Script<'a> = (ScriptCode<'a>, ScriptComment<'a>);

/// The type representing `BTreeMap<ScriptName<'a>, Script<'a>>`.
pub type ScriptMap<'a> = BTreeMap<ScriptName<'a>, Script<'a>>;

/// The type representing aliasses.
type Alias = char;

/// The type for alias names.
pub enum AliasName<'a> {
    Yes(ScriptName<'a>),
    No,
}

/// The type representing `BTreeMap<Alias, AliasName<'a>>`.
pub type AliasMap<'a> = BTreeMap<Alias, AliasName<'a>>;

/// The error type for script adding operations.
pub enum AddError {
    CodeIsEmpty,
    NameIsEmpty,
    NameIsUsed,
}

/// The database that stores scripts.
pub struct Db<'a> {
    script_map: ScriptMap<'a>,
    alias_map: AliasMap<'a>,
}

impl<'a> Db<'a> {
    /// Creates a new empty `Db`.
    ///
    pub fn new() -> Db<'a> {
        Db {
            script_map: ScriptMap::new(),
            alias_map: AliasMap::new(),
        }
    }

    /// Adds a new script with given name, code, and comment.
    ///
    pub fn add(
        &mut self,
        name: ScriptName<'a>,
        script_code: ScriptCode<'a>,
        script_comment: Option<ScriptComment<'a>>,
    ) -> Result<(), AddError> {
        let name = name.trim();
        let script_code = script_code.trim();
        let script_comment = script_comment.unwrap_or("").trim();

        let script_comment = if script_comment.is_empty() {
            "no comment given"
        } else {
            script_comment
        };

        let alias = match name.chars().nth(0) {
            Some(ch) => ch,
            None => return Err(AddError::NameIsEmpty),
        };

        if script_code.is_empty() {
            return Err(AddError::CodeIsEmpty);
        }

        if self
            .script_map
            .insert(name, (script_code, script_comment))
            .is_some()
        {
            return Err(AddError::NameIsUsed);
        }

        match self.alias_map.get(&alias) {
            Some(AliasName::Yes(_)) => {
                self.alias_map.insert(alias, AliasName::No);
            }
            None => {
                self.alias_map.insert(alias, AliasName::Yes(name));
            }
            _ => (),
        };

        Ok(())
    }
}
