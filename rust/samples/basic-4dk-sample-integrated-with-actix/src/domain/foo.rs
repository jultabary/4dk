use uuid::Uuid;

pub struct Foo {
    id: Uuid,
    title: String
}

impl Foo {
    pub fn new(id: Uuid, title: String) -> Foo {
        Foo {
            id,
            title
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }
}
