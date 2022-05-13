/// this struct use for display a item on home page.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleArticle {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub author_id: i32,
    pub create_date: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    pub id: i32,
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleUser {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub email: String,
}