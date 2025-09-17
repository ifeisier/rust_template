# Rust 项目模板

可以基于这个项目构建新的应用程序.

## 目录介绍

- `config`: 配置文件.
- `crates`: 核心代码.
- `docs`: 文档说明.
- `examples`: 代码示例.
- `scripts`: 脚本.
- `tests`: 测试代码.
- `.env.development`: 开发环境变量.

## crates 中模块的依赖关系

1. core: 业务核心逻辑 (Domain + Application).

   如果要访问外部系统, 那么要提供 `trait` 并由 `ffi` 模块实现.

2. ffi: 访问外部系统 (FFI, gRPC, 数据库访问, HTTP 请求等).

   可以依赖 `core`, 但是只能使用 `core` 中提供的 `trait`.

3. interfaces: 暴露给外部的接口, 如 Web 服务.

   可以依赖 `core` 和 `ffi`.

   注: 该模块负责启动应用程序, 并配置依赖关系等.

4. shared: 共享库, 可以被所有模块依赖.
