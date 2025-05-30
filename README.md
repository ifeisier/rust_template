# rust_template

这是一个简单的 rust 模板项目.


## 模块依赖关系

- domain(领域模块): 不依赖其它模块, 只包含每个领域的核心逻辑, 不包含具体的业务逻辑.
- infrastructure(基础设施模块): 只用来和外部系统交互, 并且只能依赖 domain 中的 traits 模块实现现具体 trait.
- interfaces(接口模块): 用来指定暴露给外界的接口和具体的业务逻辑, 也是只能依赖 domain 中的 traits 模块来调用功能.
- shared(共享模块): 包含一些通用工具类等.
- app_shared(应用共享模块): 用来加载配置、创建实例等.
