use alloc::string::String;

pub struct ShouldRunParams {
    role: String,
    action: String
}

impl ShouldRunParams {
    pub fn new(role: String, action: String) -> Self {
        Self { role, action }
    }

    pub fn role(&self) -> &String { &self.role }
    pub fn action(&self) -> &String { &self.action }
}