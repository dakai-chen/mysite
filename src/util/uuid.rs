use uuid::Uuid;

pub fn v4() -> String {
    Uuid::new_v4().simple().to_string()
}
