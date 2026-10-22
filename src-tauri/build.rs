// Icons are committed as base64 text (see `icons/*.b64`) and materialised here so
// the repository stays text-only; Tauri's bundler needs the real PNG/ICO files.
fn decode(input: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let (mut buf, mut bits) = (0u32, 0u32);
    for b in input.bytes() {
        let v = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => continue,
        } as u32;
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    out
}

fn main() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("icons");
    for (name, data) in [
        ("32x32.png", include_str!("icons/32x32.png.b64")),
        ("128x128.png", include_str!("icons/128x128.png.b64")),
        ("128x128@2x.png", include_str!("icons/128x128@2x.png.b64")),
        ("icon.png", include_str!("icons/icon.png.b64")),
        ("icon.ico", include_str!("icons/icon.ico.b64")),
    ] {
        let path = dir.join(name);
        if !path.exists() {
            std::fs::write(&path, decode(data)).expect("write icon");
        }
    }
    tauri_build::build()
}
