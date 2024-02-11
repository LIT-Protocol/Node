use uuid::Uuid;

pub fn unsafe_short_uuid() -> String {
    Uuid::new_v4().simple().to_string().get(0..8).unwrap().to_string()
}
