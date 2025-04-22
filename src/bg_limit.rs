use crate::config::Config;
pub fn limit_background(config: &Config) {
    use sysinfo::{System, SystemExt, ProcessExt};
    
    let mut system = System::new_all();
    system.refresh_all();
    
    for (pid, process) in system.processes() {
        if config.background_processes.contains(&process.name()) {
            if let Err(e) = set_process_priority(*pid, config.background_priority) {
                eprintln!("设置进程{}优先级失败: {}", pid, e);
            }
            
            if let Err(e) = limit_process_cpu(*pid, config.max_cpu_usage) {
                eprintln!("限制进程{}CPU使用率失败: {}", pid, e);
            }
        }
    }
}