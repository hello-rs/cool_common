use anyhow::anyhow;
use anyhow::Result;
use encoding::{all::GB18030, DecoderTrap, Encoding};

/// gb18030 编码转 String
pub fn gb18030_to_string(input: &[u8]) -> Result<String> {
    GB18030
        .decode(input, DecoderTrap::Strict)
        .map_err(|err| anyhow!(err))
}
