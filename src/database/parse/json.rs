use crate::{
    database::{
        errors::{FormatError, ParseError},
        Database,
    },
    script::Script,
};

impl<'a> Database<'a> {
    /// Creates a database by searching for the script names and commands inside `script_identifier` object in `json_string`.
    pub(super) fn from_json_object(
        json_string: &'a str,
        script_identifier: &'static str,
        file_name: &'static str,
    ) -> Result<Database<'a>, ParseError> {
        // Create a new database.
        let mut db = Database::new();

        // Collect all the characters of `json_string`.
        let chars = json_string.chars().collect::<Vec<char>>();

        // Create a buffer to hold tuples of a key or a value, and the line number.
        let mut buf: Vec<(&'a str, usize)> = vec![];

        // Define a variable to hold the current line number.
        let mut line_no = 1;

        // Define a variable to hold a boolean which is based on if the current character is inside quotes.
        let mut is_inside_quotes = false;

        // Define a variable to hold a boolean which is based on if the current character is inside the curly brackets of script identifier object.
        let mut is_inside_identifier = false;

        // Sum of UTF-8 length of all the characters from the beginning of the text, to.
        let mut current_index = 0;

        // When a quote symbol is found, variable below is equal to `current_index`.
        let mut start_index = 0;

        // Run a for loop for each index of characters.
        for i in 0..chars.len() {
            // The character at the current index.
            let ch = chars[i];

            // Increment `current_index` based on the lenght of the character at the current index.
            current_index += ch.len_utf8();

            // Get previous character.
            let prev_ch = if i > 0 { chars.get(i - 1) } else { None };

            // Match the current character.
            match ch {
                '"' => {
                    if prev_ch != Some(&'\\') {
                        // Change `is_inside_quotes`.
                        is_inside_quotes = !is_inside_quotes;

                        if is_inside_quotes {
                            // That means current index is the beginning of a key or a value.
                            start_index = current_index;
                        } else {
                            // Get a reference to the string between `start_index` and `current_index - 1`
                            // We decrement current_index by one, cuz it shouldn't include `"`, the current character.
                            let key_or_value = &json_string[start_index..current_index - 1];

                            // Now we can set `start_index` to 0.
                            start_index = 0;

                            // If the current character is inside script identifier object, push `key_or_value` and the line number to `buf`.
                            if is_inside_identifier {
                                buf.push((key_or_value, line_no));
                            }

                            // If `key_or_value` equal given `script_identifier`.
                            if key_or_value == script_identifier {
                                is_inside_identifier = true;
                            }
                        }
                    }
                }

                '}' => {
                    // If the current character is not inside nor quotes, neither script identifier object, break the loop.
                    if !is_inside_quotes && is_inside_identifier {
                        break;
                    }
                }

                '\n' => {
                    // Increment the current line number by one.
                    line_no += 1;
                }

                // If the current character is another thing, do nothing.
                _ => (),
            }
        }

        // Run a for loop for each index of the buffer.
        for i in 0..buf.len() {
            // Only do below if the current index is divisible by 2.
            if i % 2 == 0 {
                // Get a reference to the script name and its line number at current index.
                let (mut name, name_line_no) = buf[i];

                // Remove leading and trailing spaces.
                name = name.trim();

                // Try to get the script command.
                let command = match buf.get(i + 1) {
                    // If there is a script command, remove its leading and trailing spaces.
                    Some((command, _)) => command.trim(),
                    // If there isn't a script command, return an error.
                    None => {
                        return Err(FormatError::NoCommand.into_parse_error(name_line_no, file_name))
                    }
                };

                // Try to add the script name and script to the database.
                match db.add(name, Script::new(command)) {
                    // Do nothing, if it's OK.
                    Ok(()) => (),
                    // Return an error, if there is a `FormatError`.
                    Err(err) => return Err(err.into_parse_error(name_line_no, file_name)),
                };
            }
        }

        // Succesfully return the database.
        Ok(db)
    }
}
