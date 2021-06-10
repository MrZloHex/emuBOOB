pub struct Compile {
    filename: String,
}

impl Compile {
    pub fn new (filename: String) -> Compile {
        Compile{filename: filename}
    }
}