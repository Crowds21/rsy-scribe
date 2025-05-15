use serde_bytes::ByteBuf;

pub struct Lexer {
    input: ByteBuf,
    length: usize,
    offset: usize,
    width: usize,
}
