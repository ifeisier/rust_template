//! 相关实体类

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}
