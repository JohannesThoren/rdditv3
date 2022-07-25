use serde;
mod post;
mod subreddit;
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize, __private::de::IdentifierDeserializer};
    use serde_json;
    use std::fs;

    use crate::subreddit;

    #[test]
    fn test_deserialize() {
        let data = fs::read_to_string("./sweden.json").expect("unable to read file");
        let sr: subreddit::Subreddit =
            serde_json::from_str(&data).expect("JSON does not have correct format");

        println!("{:#?}", sr);
    }
}
