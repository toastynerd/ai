use sled;

pub fn store_prompt(db: &sled::Db, prompt: &crate::prompt::Prompt) {
    db.insert(prompt.name.to_string(), crate::prompt::serialize_prompt(prompt).as_bytes()).unwrap();
}

pub fn get_prompt(db: &sled::Db, name: &str) -> crate::prompt::Prompt {
    let prompt = db.get(name).unwrap().unwrap();
    crate::prompt::deserialize_prompt(std::str::from_utf8(&prompt).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_prompt() {
        let db = sled::open("db/test_store").unwrap();
        let prompt = crate::prompt::create_prompt("joke".to_string(), "tell me a joke".to_string());
        store_prompt(&db, &prompt);
        assert_eq!(db.get("joke").unwrap().unwrap(), crate::prompt::serialize_prompt(&prompt));
        db.clear().unwrap();
    }

    #[test]
    fn test_get_prompt() {
        let db = sled::open("db/test_get").unwrap();
        let stored = crate::prompt::create_prompt("joke".to_string(), "tell me a joke".to_string());
        store_prompt(&db, &stored);
        let retrieved = get_prompt(&db, "joke");
        assert_eq!(stored.name, retrieved.name);
        assert_eq!(stored.template, retrieved.template);
        db.clear().unwrap();
    }
}
