use super::dictionary;

pub struct Compile {
    filename: String,
    dictionary: dictionary::Dictionary
}

impl Compile {
    pub fn new (filename: String) -> Compile {
        Compile{
            filename: filename,
            dictionary: dictionary::Dictionary::new()
        }
    }
}