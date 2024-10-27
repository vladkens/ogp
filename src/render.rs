use enum_map::{enum_map, Enum, EnumMap};
use maud::{html, Markup};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::utils::safe_id;

#[derive(Debug, Deserialize, Serialize, Enum, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OGTheme {
  Default,
  NightOwl,
  Github,
  Matrix,
  Dracula,
  Tinacious,
  ShadesOfPurple,
}

#[derive(Debug)]
struct Colors<'a> {
  background: &'a str,
  author: &'a str,
  title: &'a str,
  url: &'a str,
}

static COLORS: Lazy<EnumMap<OGTheme, Colors<'static>>> = Lazy::new(|| {
  // https://github.com/sagarhani/og-image-generator/blob/main/utils/themes.ts#L7
  enum_map! {
    OGTheme::Default => Colors { background: "#000000", title: "#ffffff", author: "#ffffff", url: "#ffffff" },
    OGTheme::NightOwl => Colors { background: "#011627", title: "#c792ea", author: "#f07178", url: "#82aaff" },
    OGTheme::Github => Colors { background: "#f0f0f0", title: "#2f363c", author: "#005cc5", url: "#d73a49" },
    OGTheme::Matrix => Colors { background: "#003300", title: "#00ff00", author: "#ccffcc", url: "#ff6666" },
    OGTheme::Dracula => Colors { background: "#191a21", title: "#bd93f9", author: "#ff79c6", url: "#f8f8f2" },
    OGTheme::Tinacious => Colors { background: "#ececec", title: "#ff3399", author: "#00aee8", url: "#44425e" },
    OGTheme::ShadesOfPurple => Colors { background: "#2d2b55", title: "#b362ff", author: "#ff9d00", url: "#9effff" },
  }
});

#[derive(Debug, Deserialize)]
pub struct OGImage {
  pub title: String,
  pub photo: String,
  pub author: String,
  pub url: String,
  pub theme: OGTheme,
}

impl OGImage {
  pub fn default() -> Self {
    Self {
      title: "Dynamic Open Graph Image Generator".to_string(),
      photo: "https://avatars.githubusercontent.com/u/825754".to_string(),
      author: "vladkens".to_string(),
      url: "vnotes.pages.dev".to_string(),
      theme: OGTheme::Default,
    }
  }

  fn c(&self) -> &Colors {
    &COLORS[self.theme.clone()]
  }

  fn multiline_text(&self, text: &str, x: u32, y: u32, font_size: u32) -> Markup {
    let lines = textwrap::wrap(text, 24);
    let lines = match lines.len() > 3 {
      false => lines,
      true => {
        let mut lines = lines[..3].to_vec().clone();
        lines[2] = format!("{}…", lines[2]).try_into().unwrap();
        lines
      }
    };

    let dy = (font_size as f32 * 1.25) as u32;

    html!(text x=(x) y=(y) font-size=(font_size) fill=(self.c().title) font-weight="700" {
      @for line in &lines { tspan x=(x) dy=(dy) { (line) } }
    })
  }

  fn circle_avatar(&self, url: &str, cx: u32, cy: u32, radius: u32) -> Markup {
    let clip_id = safe_id();

    html!({
      defs {
        clipPath id=(clip_id) { circle cx=(cx) cy=(cy) r=(radius) {} }
      }

      g clip-path=(format!("url(#{clip_id})")) {
        image href=(url) x=(cx-radius) y=(cy-radius) width=(radius * 2) height=(radius * 2) {}
        circle cx=(cx) cy=(cy) r=(radius) stroke=(self.c().title) stroke-width="4" fill="none" {}
      }
    })
  }

  pub fn render_svg(&self) -> Markup {
    let (w, h) = (1200, 630);

    let pic_r = 50;
    let pic_y = h - 128;
    let l1_y = pic_y - (pic_r - 10) / 2 + 48 / 2 - 6;
    let l2_y = pic_y - (pic_r - 10) / 2 + 32 / 2 + 6;

    html!(svg xmlns="http://www.w3.org/2000/svg" viewBox=(format!("0 0 {} {}", w, h)) width=(w) height=(h) {
      style { (
        r#"
          text { font-family: 'Open Sans', Arial, sans-serif; }
        "#.trim())
      }

      rect x="0" y="0" width=(w) height=(h) fill=(self.c().background) stroke=(self.c().title) stroke-width="16" {}
      (self.multiline_text(&self.title, 128 - 50/2, 128, 72))
      text x=(128 + 50 + 20) y=(l1_y) dominant-baseline="auto" font-weight="700" fill=(self.c().author) font-size="48" { (&self.author) }
      text x=(128 + 50 + 20) y=(l2_y) dominant-baseline="hanging" font-weight="400" fill=(self.c().url) font-size="32" { (&self.url) }
      (self.circle_avatar(&self.photo, 128, pic_y, pic_r))
    })
  }

  pub fn render_png(&self) -> Vec<u8> {
    use resvg::{tiny_skia, usvg};

    let tree = {
      let mut opt = usvg::Options::default();
      opt.fontdb_mut().load_system_fonts();
      usvg::Tree::from_str(&self.render_svg().into_string(), &opt).unwrap()
    };

    let size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap.encode_png().unwrap()
  }
}
