use chrono::{prelude::*, LocalResult, ParseResult};
use std::time::{Duration, Instant};

/// [`%Y-%m-%dT%H:%M:%S%.f`](chrono::format::strftime)
const DEFAULT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// 开始测量运行时间
#[inline]
pub fn start_elapsed() -> Instant {
    Instant::now()
}

/// 结束测量运行时间
#[inline]
pub fn end_elapsed(now: Instant) -> Duration {
    now.elapsed()
}

/// 获取当前时间
#[inline]
pub fn now() -> DateTime<Local> {
    Local::now()
}

/// 获取当前时间戳
#[inline]
pub fn timestamp() -> i64 {
    Local::now().timestamp()
}

/// 获取当前时间字符串,默认(%Y-%m-%d %H:%M:%S)
#[inline]
pub fn string() -> String {
    Local::now().format(DEFAULT_FORMAT).to_string()
}

/// 时间戳转 time
pub fn timestamp_to_time(timestamp: i64) -> LocalResult<DateTime<Local>> {
    Local.timestamp_opt(timestamp, 0)
}

/// 字符串转时间
#[inline]
pub fn string_to_time(s: &str) -> ParseResult<DateTime<Local>> {
    Local.datetime_from_str(s, DEFAULT_FORMAT)
}
