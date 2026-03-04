use uuid::Uuid;

pub struct Subscription {
    pub id: String,
    pub student_id: String,
}

impl Subscription {
    pub fn new(student_id: &str) -> Subscription {
        Self {
            id: Uuid::now_v7().to_string(),
            student_id: student_id.to_string(),
        }
    }
}