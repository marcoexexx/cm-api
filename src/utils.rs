use std::{io::Write, os::unix::fs::FileExt};

pub fn decode_base64(encoded: &str) -> Option<Vec<u8>> {
  let mut result = Vec::new();
  let mut buffer = 0;
  let mut leftover = 0;

  for c in encoded.chars() {
    if c == '=' {
      break;
    }

    let value = match c {
      'A'..='Z' => c as u8 - b'A',
      'a'..='z' => c as u8 - b'a' + 26,
      '0'..='9' => c as u8 - b'0' + 52,
      '+' => 62,
      '/' => 63,
      _ => return None,
    };

    buffer = (buffer << 6) | value as u32;
    leftover += 6;

    if leftover >= 8 {
      leftover -= 8;
      result.push((buffer >> leftover) as u8);
      buffer &= (1 << leftover) - 1;
    }
  }

  Some(result)
}

pub fn write_html(content: &scraper::Html) {
  let mut file = std::fs::File::options()
    .create(true)
    .write(true)
    .open("_debug_content.html")
    .unwrap();

  let html = content.html();

  file.write_all(html.as_bytes()).unwrap();
}
