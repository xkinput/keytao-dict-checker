/// 编码最大长度
pub(crate) const MAX_CODE_LEN: usize = 6;

/// 是否为形码
fn is_xm(b: &&u8) -> bool {
    matches!(b, b'a' | b'i' | b'o' | b'u' | b'v')
}

/// 编码的形码段长度
fn xm_len(code: &str) -> usize {
    code.as_bytes().iter().rev().take_while(is_xm).count()
}

/// 编码的音码段长度
fn ym_len(code: &str) -> usize {
    code.len() - xm_len(code)
}

/// 编码的音码段切片
pub(crate) fn ym(code: &str) -> &[u8] {
    &code.as_bytes()[..ym_len(code)]
}

/// 所有合法的更短编码
pub(crate) fn shorter(code: &str) -> impl Iterator<Item = &str> {
    (ym_len(code)..code.len()).map(|len| &code[..len])
}

/// 将单字编码的构词码打包
pub(crate) fn stem_key(code: &str) -> Option<u32> {
    match code.as_bytes() {
        [a, b, c, ..] => Some(u32::from_le_bytes([*a, *b, *c, 0])),
        _ => None,
    }
}

/// 解包构词码并取出音码段
pub(crate) fn stem_ym(stem: u32) -> [u8; 2] {
    let [a, b, _, _] = stem.to_le_bytes();
    [a, b]
}

/// 将词组编码的音码段打包
pub(crate) fn ym_key(code: &str) -> Option<u32> {
    match ym(code) {
        [a, b, c] => Some(u32::from_le_bytes([*a, *b, *c, 0])),
        [a, b, c, d] => Some(u32::from_le_bytes([*a, *b, *c, *d])),
        _ => None,
    }
}

/// 词组的音码段长度
pub(crate) fn phrase_ym_len(text: &str) -> usize {
    let mut cs = text.chars().skip(2);
    match (cs.next(), cs.next()) {
        (Some(_), None) => 3,
        _ => 4,
    }
}

/// 伴生的音码（飞键）
pub(crate) fn full_alt_target([a, b]: [u8; 2]) -> Option<[u8; 2]> {
    match (a, b) {
        (b'f', b'e') => Some([b'q', b'e']),
        (b'q', b'e') => Some([b'f', b'e']),
        (b'f', b'z') => Some([b'q', b'z']),
        (b'q', b'z') => Some([b'f', b'z']),
        (b'j', b'e') => Some([b'w', b'e']),
        (b'w', b'e') => Some([b'j', b'e']),
        (b'j', b'z') => Some([b'w', b'z']),
        (b'w', b'z') => Some([b'j', b'z']),
        (b'e', b'm') => Some([b'e', b'x']),
        (b'e', b'x') => Some([b'e', b'm']),
        (b'f', b'm') => Some([b'f', b'x']),
        (b'f', b'x') => Some([b'f', b'm']),
        (b'g', b'm') => Some([b'g', b'x']),
        (b'g', b'x') => Some([b'g', b'm']),
        (b'h', b'm') => Some([b'h', b'x']),
        (b'h', b'x') => Some([b'h', b'm']),
        (b'k', b'm') => Some([b'k', b'x']),
        (b'k', b'x') => Some([b'k', b'm']),
        (b'w', b'm') => Some([b'w', b'x']),
        (b'w', b'x') => Some([b'w', b'm']),
        (b'f', b'h') => Some([b'q', b'h']),
        _ => None,
    }
}

/// 若飞键位于首码，则返回飞键
pub(crate) fn first_alt_target(seg: [u8; 2]) -> Option<u8> {
    let [t, _] = full_alt_target(seg)?;
    (t != seg[0]).then_some(t)
}
