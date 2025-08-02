//! 值对象
//!
//! - 没有唯一标识.
//! - 用来当参数或返回值传递, 并且不可变.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zipcode: String,
}