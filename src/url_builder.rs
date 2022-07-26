const URLROOT: &str = "https://reddit.com/r";
#[derive(Debug)]

pub enum Sorting {
    BEST,
    HOT,
    NEW,
    TOP,
    RISING,
}

impl Sorting {
    pub fn get(&self) -> String {
        return match &self {
            Sorting::BEST => "best".to_string(),
            Sorting::HOT => "hot".to_string(),
            Sorting::NEW => "new".to_string(),
            Sorting::TOP => "top".to_string(),
            Sorting::RISING => "rising".to_string(),
        };
    }

    pub fn from_string(s: String) -> Sorting {
        return match s.as_str() {
            "best" => Sorting::BEST,
            "hot" => Sorting::HOT,
            "new" => Sorting::NEW,
            "top" => Sorting::TOP,
            "rising" => Sorting::RISING,
            _ => Sorting::NEW,
        };
    }
}
#[derive(Debug)]
pub struct URLBuilder {
    pub subreddit: String,
    pub sorting: Option<Sorting>,
    pub limit: Option<u8>,
}
impl URLBuilder {
    pub fn new(subreddit: String, sorting: Option<Sorting>, limit: Option<u8>) -> Self {
        URLBuilder {
            subreddit,
            sorting,
            limit,
        }
    }

    pub fn build(&self) -> String {
        let mut url = format!("{}/{}", URLROOT, &self.subreddit);

        match &self.sorting {
            Some(s) => url = format!("{}/{}.json?", url, s.get()),
            None => url = format!("{}/.json?", url),
        }

        match &self.limit {
            Some(l) => url = format!("{}&limit={}", url, l),
            None => url = url,
        }

        return url;
    }
}
