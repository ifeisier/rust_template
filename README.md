# rust_template

这是一个简单的 rust 模板项目.


## 模块依赖关系

- domain(领域模块): 只包含每个领域的核心逻辑, 不包含具体的业务逻辑.
- application(应用模块): 协调所有领域逻辑, 实现具体的业务逻辑.
- infrastructure(基础设施模块): 只用来和外部系统交互.
- interfaces(接口模块): 用来指定暴露给外界的接口.
- shared(共享模块): 包含一些通用工具类等.


## 依赖方向图

```
         domain
           ▲
           |
  application (依赖 domain)
           ▲
           |
    interfaces（API）──► 调用 application
           ▲
           |
   infrastructure（实现 domain 中的接口）
```