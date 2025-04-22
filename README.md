# Fuu

安卓视频优化工具

## 功能
- 监控高CPU占用的视频应用进程
- 绑定视频应用进程到指定CPU核心
- 限制视频应用CPU使用率
- 优化视频应用进程优先级

## 使用说明
1. 安装Rust环境
2. 克隆本项目
3. 运行`cargo build --release`
4. 修改`config/module.conf`配置文件
5. 运行编译后的程序

## 配置示例
```
[video]
priority = 3
max_cpu_usage = 0.7
cpu_threshold = 70.0
bind_cores = [2, 3]
```

## 贡献指南
欢迎提交PR和Issue

## 许可证
MIT License