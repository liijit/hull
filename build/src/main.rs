mod ser;
mod external;
mod render;

use crate::render::*;

use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::env;

use ser::{SublimeTheme, Rules, Variables};

fn main() {
	let theme = construct_theme();

  let mut path = PathBuf::new();
  path.push(env::current_dir().unwrap().parent().unwrap());
  path.push("Hull (Base).hidden-theme");

	let mut buf = Vec::new();
  buf.extend(b"// THIS FILE IS GENERATED");
  buf.extend(b"\n");
  buf.extend(serde_json::to_vec_pretty(&theme).unwrap());

  let _ = write_file(path.to_str().unwrap(), buf.as_slice());

  render_image();
}

fn write_file(dir: &str, s: &[u8]) -> std::io::Result<()> {
  let mut some_file = File::create(dir)?;
  some_file.write(s)?;
  Ok(())
}

fn construct_theme() -> SublimeTheme {
	SublimeTheme {
		variables: Variables::new(),
		rules: Rules::new(),
	}
}
