use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Prompt {
    name: String,
    template: String,
    number_of_parameters: usize,
}

fn create_prompt(name: String, template: String) -> Prompt {
    let num_params = template.matches(|c| c == '{').count();
    Prompt {
        name: name,
        template: template,
        number_of_parameters: num_params,
    }
}

fn serialize_prompt(prompt: &Prompt) -> String {
    serde_json::to_string(prompt).unwrap()
}

fn deserialize_prompt(prompt: &str) -> Prompt {
    serde_json::from_str(prompt).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_prompt_no_params() {
        let my_prompt = create_prompt("joke".to_string(), "tell me a joke".to_string());
        assert_eq!(my_prompt.name, "joke");
        assert_eq!(my_prompt.template, "tell me a joke");
        assert_eq!(my_prompt.number_of_parameters, 0);
    }

    #[test]
    fn test_create_prompt_with_params() {
        let my_prompt = create_prompt("joke".to_string(), "tell me a joke as {name}".to_string());
        assert_eq!(my_prompt.number_of_parameters, 1);
    }

    #[test]
    fn test_serialize_prompt() {
        let test_prompt = create_prompt("joke".to_string(), "tell me a joke as {name}".to_string());
        let serialized = serialize_prompt(&test_prompt);
        assert_eq!(serialized, "{\"name\":\"joke\",\"template\":\"tell me a joke as {name}\",\"number_of_parameters\":1}");
    }

    #[test]
    fn test_deserialize_prompt() {
        let serialized = "{\"name\":\"joke\",\"template\":\"tell me a joke as {name}\",\"number_of_parameters\":1}";
        let deserialized = deserialize_prompt(serialized);
        assert_eq!(deserialized.name, "joke");
        assert_eq!(deserialized.template, "tell me a joke as {name}");
        assert_eq!(deserialized.number_of_parameters, 1);
    }
}
