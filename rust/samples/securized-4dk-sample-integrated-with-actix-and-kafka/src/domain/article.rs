pub struct Article {
    title: String,
    body: String,
}

impl Article {
    pub fn new(title: String, body: String) -> Article {
        Article {
            title,
            body
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_body(&self) -> &String {
        &self.body
    }
}