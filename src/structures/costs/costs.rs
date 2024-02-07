pub struct Costs {
    pub amount: f32,
    pub comments: Option<String>,
}

impl Costs {
    pub fn new(amount: f32, comments: Option<String>) -> Self {
        Self {
            amount,
            comments,
        }
    }
}