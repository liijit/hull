mod ser;
mod external;
mod render;

use crate::render::*;

use std::fs::File;
use std::io::Write;

use ser::{SublimeTheme, Rules, Variables};

fn main() {
	let theme = construct_theme();

  let mut path = crate::dir::PathConstructor::new();
  path.set_parent_directory(crate::dir::PathLocations::ProjectRoot, "");
  path.set_filename("Hull (Base)");
  path.set_extension("hidden-theme");

	let mut buf = Vec::new();
  buf.extend(b"// THIS FILE IS GENERATED");
  buf.extend(b"\n");
  buf.extend(serde_json::to_vec_pretty(&theme).unwrap());

  let _ = write_file(path.get_full_path().to_str().unwrap(), buf.as_slice());

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

pub struct Asset {
  texture: crate::Texture,
  margin: crate::Margin,
  stroke: crate::Stroke,
  radius: crate::Radius,
  scale: crate::Scale
}

mod dir {
  use std::path::PathBuf;
  use std::fs::DirBuilder;
  use crate::Asset;
  use std::borrow::Cow;
  use std::env;

  #[derive(Debug, Clone)]
  pub struct PathConstructor {
    path: PathBuf,
    state: BufState,
    filename: String,
    prefix: u32,
    extension: String,
  }

  pub enum PathLocations {
    Assets,
    ProjectRoot
  }

  #[derive(Debug, PartialEq, Clone)]
  enum BufState {
    None,
    Dir,
    DirFile,
    DirFileExt
  }

  impl<'a> PathConstructor {
    pub fn new() -> Self {
      Self {
        path: PathBuf::new(),
        state: BufState::None,
        filename: String::new(),
        prefix: 1,
        extension: String::new(),
      }
    }

    pub fn set_filename(&mut self, s: &'a str) {
      self.filename = s.to_string();
    }

    pub fn set_extension(&mut self, s: &'a str) {
      self.extension = s.to_string();
    }

    pub fn set_asset_filename(&mut self, asset: &Asset) {
      self.filename = format!("{}_{}_{}_{}{}", &asset.texture, &asset.margin, &asset.stroke, &asset.radius, &asset.scale);
    }

    pub fn set_parent_directory(&mut self, p: PathLocations, c: &str) {
      if self.state != BufState::None {
        self.path.clear()
      }

      self.path.push(env::current_dir().unwrap().parent().unwrap().to_path_buf());
      match p {
        PathLocations::Assets => {
          self.path.push("assets")
        }
        PathLocations::ProjectRoot => {},
      }
      self.path.push(c);
      self.state = BufState::Dir;
    }

    pub fn get_filename(&'a self) -> Cow<'a, str> {
      Cow::from(&self.filename)
    }

    pub fn get_path_buf(&mut self) -> PathBuf {
      if self.state == BufState::DirFileExt || self.state == BufState::DirFile {
        self.path.parent().unwrap().to_path_buf()
      } else {
        self.path.to_path_buf()
      }
    }

    pub fn set_prefix(&mut self, s: u32) {
      self.prefix = s;
    }

    pub fn get_full_path(&'a mut self) -> PathBuf {
      if self.state == BufState::DirFileExt {
        self.path.pop();
      }
      // needs benchmark, rust would otherwise guess reserve space
      // there's surely a better way with slices or a crate for less frequent
      // allocations
      self.path.reserve(1+&self.filename.len() + &self.extension.len());
      // push an item on vec then append it with name + ext
      self.path.push("r");
      if &self.prefix < &2 {
        self.path.set_file_name(&self.filename);
      } else {
        self.path.set_file_name(format!("{}@{}x", &self.filename, &self.prefix));
        self.set_prefix(1);
      }
      self.path.set_extension(&self.extension);
      self.state = BufState::DirFileExt;
      self.path.to_path_buf()
    }
  }

  pub trait PathBuilder {
    fn create_dir(&self) {}
  }

  impl PathBuilder for PathConstructor {
    fn create_dir(&self) {
      if !self.path.try_exists().unwrap() {
        DirBuilder::new().recursive(true).create(self.path.as_path()).unwrap();
      }
    }
  }
}
