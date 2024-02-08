pub struct Costs {
    pub amount: f32,
    pub comments: Option<String>,
    pub timestamp: String,
}

impl Costs {
    pub fn new(amount: f32, comments: Option<String>, timestamp: String) -> Self {
        Self {
            amount,
            comments,
            timestamp
        }
    }
}
