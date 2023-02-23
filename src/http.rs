pub struct Http {
    r: char,
    n: char,
}

impl Http {
    pub fn new() -> Http {
        Http { r: '\r', n: '\n' }
    }
    fn serialize(&self, data: &str) -> String {
        format!(
            "{}{}{}{}{}{}",
            data.len(),
            self.r,
            self.n,
            data,
            self.r,
            self.n
        )
    }
    pub fn chunk(&self, data: &str) -> String {
        self.serialize(data)
    }
}
