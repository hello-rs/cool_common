use std::io;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};

use crate::config::C_CONFIG;

pub fn init() {
    // 创建一个每天轮换的日志写入者。
    let file_appender =
        tracing_appender::rolling::daily(&C_CONFIG.tracing.dir, &C_CONFIG.tracing.name);
    // 创建一个非阻塞的线程外的日志写入者。
    // _guard 守卫 non_blocking_appender 日志写入者
    // 当被 drop 时,剩余的日志将被 flushed
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);

    // 通过配置中的日志级别初始化
    let env = EnvFilter::try_new(&C_CONFIG.tracing.level).unwrap();
    // 输出到文件中
    let file_layer = tracing_subscriber::fmt::layer().with_writer(non_blocking_appender);
    if C_CONFIG.tracing.console {
        // 输入到控制台中
        let stdout = tracing_subscriber::fmt::layer().with_writer(io::stdout);
        Registry::default()
            .with(env)
            .with(file_layer)
            .with(stdout)
            .init();
    } else {
        Registry::default().with(env).with(file_layer).init();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
