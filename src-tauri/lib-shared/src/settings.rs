use shared::Result;

struct settings {
    directory: String,
    file: String,
}

impl settings {

    pub fn new(directory: &str, file: &str) -> settings {
        settings { directory, file }
    }

    pub fn read(&self) -> Result<String> {
        let mut file = File::open(&self.location)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn write(&self, contents: &str) -> Result<()> {
        let mut file = File::create(&self.location)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn path(&self) -> String {
        format!("{}/{}", self.directory, self.file)
    }
}