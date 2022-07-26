use serde;

mod post;
mod response;
mod http;
mod url_builder;
#[cfg(test)]
mod tests {
    use crate::{response, url_builder};
    use reqwest;
    use serde::{Deserialize, Serialize, __private::de::IdentifierDeserializer};
    use serde_json;
    use std::fs;
    use tokio;

    #[tokio::test]
    async fn test_deserialize() {
        let res = reqwest::get("https://reddit.com/r/sweden/new.json").await;
        match res {
            Ok(response) => {
                let data = response.text().await.unwrap();
                let sr: response::Response = serde_json::from_str(&data.as_str())
                    .expect("JSON does not have correct format");
                println!("{:#?}", sr);
            }

            Err(_) => todo!(),
        }
    }

    #[tokio::test]
    async fn test_urlbuilder() {
        
        let ub = url_builder::URLBuilder::new(
            "sweden".to_string(),
            Some(url_builder::Sorting::NEW),
            Some(5),
        );

        println!("{:#?}", ub);
        let url = ub.build();
        println!("{}",url)
    }

    use crate::http;

    #[tokio::test]
    async fn test_fetch() {
        let ub = url_builder::URLBuilder::new(
            "Crappyoffbrands".to_string(),
            Some(url_builder::Sorting::NEW),
            Some(5),
        );
        let posts = http::fetch(ub.build()).await;
        
        for post in posts {
            println!("{:<100}{:<32}{:<200}", post.data.title, post.data.author, post.data.url)
        }


    } 
}
