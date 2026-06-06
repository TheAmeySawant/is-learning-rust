pub fn normal_to_pig_latin(input: &str) -> String {
    let mut result = Vec::new();

    for word in input.split_whitespace() {
        let mut chars = word.chars();

        if let Some(first) = chars.next() {
            let lower_first = first.to_lowercase().next().unwrap();

            let pig_word = match lower_first {
                'a' | 'e' | 'i' | 'o' | 'u' => format!("{word}hay"),
                _ => {
                    let rest: String = chars.collect();
                    format!("{rest}{first}ay")
                }
            };

            result.push(pig_word);

        }
    }

    result.join(" ")
}
