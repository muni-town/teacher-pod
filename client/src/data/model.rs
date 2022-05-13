/// this struct use for display a item on home page.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleArticle {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub author_id: i32,
    pub create_date: i64,
}