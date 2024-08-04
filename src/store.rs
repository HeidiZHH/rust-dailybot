use tokio::io;


#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Item {
    id: String,
    message: String,
}

impl Item {
    pub fn new(id: String, message: String) -> Item {
        Item {id: id, message: message}
    }
    pub fn message(&self) -> String {
        self.message.clone()
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Clone)]
pub struct Store;

impl Store {
    pub fn insert(&mut self, message: String) -> Result<Item, io::Error>{
        unimplemented!()
    }

    pub fn get(&self, id: String) -> Result<String, io::Error>{
        unimplemented!()
    }

    pub fn set_user_name(&mut self, username: String) -> Result<String, io::Error> {
        unimplemented!()
    }
}