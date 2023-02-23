pub struct Http {
    r: char,
    n: char,
}

impl<'a> Http {
    pub fn new() -> Http {
        Http { r: '\r', n: '\n' }
    }
    pub fn chunk(&self, data: &str) -> Vec<u8> {
        let b = format!(
            "{}{}{}{}{}{}",
            data.len(),
            self.r,
            self.n,
            data,
            self.r,
            self.n
        )
        .as_bytes();
        Vec::from(b)
    }
}
