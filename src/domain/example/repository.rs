//! 抽象出了第三方服务访问接口, 在 infrastructure 模块中实现.

use async_trait::async_trait;

/// Example 数据访问接口.
#[async_trait]
pub trait ExampleRepository: Send + Sync + 'static {

}