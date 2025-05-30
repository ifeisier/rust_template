//! 暴露给外界的接口, 比如 Web API、CLI 接口等.
//!
//! 在具体的业务逻辑中只能依赖 domain 中的 traits 模块, 来调用功能.

pub mod http;
