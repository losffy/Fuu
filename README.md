# Fuu

Windows后台进程优先级和CPU限制工具

## 功能
- 设置后台进程优先级
- 限制后台进程CPU使用率
- 监控高CPU占用进程
- 绑定进程到指定CPU核心

## 使用说明
1. 安装Rust环境
2. 克隆本项目
3. 运行`cargo build --release`
4. 修改`config/module.conf`配置文件
5. 运行编译后的程序

## 配置示例
```
[process]
background_priority = 1
max_cpu_usage = 0.5
cpu_threshold = 80.0
bind_cores = [0, 1]
```

## 贡献指南
欢迎提交PR和Issue

## 许可证
MIT License