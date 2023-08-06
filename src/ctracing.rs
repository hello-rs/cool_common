use std::io;
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};

use crate::config::C_CONFIG;

pub fn init_by_config() {
    let dir = &C_CONFIG.tracing.dir;
    let file_name = &C_CONFIG.tracing.name;
    let level = &C_CONFIG.tracing.level;
    let is_console = C_CONFIG.tracing.console;
    init(dir, file_name, level, is_console);
}

pub fn init(dir: &str, file_name: &str, level: &str, is_console: bool) {
    // 创建一个每天轮换的日志写入者。
    let file_appender = rolling::daily(dir, file_name);
    // 创建一个非阻塞的线程外的日志写入者。
    // _guard 守卫 non_blocking_appender 日志写入者
    // 当被 drop 时,剩余的日志将被 flushed
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    // 通过配置中的日志级别初始化
    let env = EnvFilter::try_new(level).unwrap();
    // 输出到文件中
    let file_layer = fmt::layer()
        .pretty()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    if is_console {
        // 输入到控制台中
        let stdout = fmt::layer().pretty().with_writer(io::stdout);

        Registry::default()
            .with(env)
            .with(file_layer)
            .with(stdout)
            .init();
    } else {
        Registry::default().with(env).with(file_layer).init();
    }
    info!("init tracing success.")
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
