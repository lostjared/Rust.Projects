struct RType {
    pub data: String,
    pub index: u32,
}

struct RTypeBuilder {
    data: Option<String>,
    index: Option<u32>,
}

impl RTypeBuilder {
    fn new() -> Self {
        Self {
            data: None,
            index: None,
        }
    }

    fn set_text(mut self, text: String) -> Self {
        self.data = Some(text);
        self
    }

    fn set_index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    fn build(self) -> RType {
        RType {
            data: self.data.expect("data not found"),
            index: self.index.unwrap_or(0),
        }
    }
}

fn main() -> std::io::Result<()> {
    let rtype = RTypeBuilder::new()
        .set_text("Hello, World".to_string())
        .set_index(10)
        .build();
    println!("data: {}, index: {}", rtype.data, rtype.index);
    Ok(())
}
