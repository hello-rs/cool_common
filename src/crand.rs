use anyhow::bail;
use anyhow::Result;
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

// const PASSWORD_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
//                             abcdefghijklmnopqrstuvwxyz\
//                             0123456789)(*&^%$#@!~";
const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789";
const CODE_CHARSET: &[u8] = b"0123456789";
pub fn gen_range<T, R>(range: R) -> Result<T>
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    if range.is_empty() {
        bail!("cannot sample empty range.")
    }
    Ok(rand::thread_rng().gen_range(range))
}

/// 随机a-zA-Z0-9
pub fn gen_string(len: usize) -> Result<String> {
    gen_string_charset(len, GEN_ASCII_STR_CHARSET)
}

/// 随机数字验证码
pub fn gen_code(len: usize) -> Result<String> {
    gen_string_charset(len, CODE_CHARSET)
}

/// 随机指定长度的指定字符集
pub fn gen_string_charset(len: usize, charset: &[u8]) -> Result<String> {
    let mut rng = rand::thread_rng();
    let res: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    Ok(res)
}
