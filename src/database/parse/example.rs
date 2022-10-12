use crate::{database::Database, script::Script};

impl<'a> Database<'a> {
    pub fn from_example() -> Database<'a> {
        let mut db = Database::new();

        #[allow(unused_must_use)]
        {
            db.add(
                "greet",
                Script::new_with_comment("echo hey!", "Prints a greeting message."),
            );
        }

        db
    }
}
