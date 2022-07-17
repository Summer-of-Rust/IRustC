pub struct Message {
    pub id: u32,
    pub room: String,
    pub name: String,
    pub content: String,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use entity::message;
    use sea_orm::Set;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
