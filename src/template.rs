pub fn render_prompt(prompt: &crate::prompt::Prompt, params: &Vec<String>) -> String {
    let mut rendered = prompt.template.clone();

    if prompt.number_of_parameters != params.len() {
        panic!("Incorrect number of parameters for prompt \"{}\". Expected {}, got {}", prompt.name, prompt.number_of_parameters, params.len());
    }

    if prompt.number_of_parameters == 0 {
        return rendered;
    }

    for (i, param) in params.iter().enumerate() {
        rendered = rendered.replace(&format!("{{{}}}", i), param);
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_prompt_no_params() {
        let test_prompt = crate::prompt::create_prompt("joke".to_string(), "tell me a joke".to_string());
        assert_eq!(render_prompt(&test_prompt, &vec![]), "tell me a joke");
    }

    #[test]
    fn test_render_prompt_with_params() {
        let test_prompt = crate::prompt::create_prompt("joke".to_string(), "tell me a joke as {0}".to_string());
        assert_eq!(render_prompt(&test_prompt, &vec!["Bob".to_string()]), "tell me a joke as Bob");
    }

    #[test]
    #[should_panic(expected = "Incorrect number of parameters for prompt \"joke\". Expected 1, got 0")]
    fn test_should_panic_with_incorrect_number_of_params() {
        let test_prompt = crate::prompt::create_prompt("joke".to_string(), "tell me a joke as {0}".to_string());
        render_prompt(&test_prompt, &vec![]);
    }
}
