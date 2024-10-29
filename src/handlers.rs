use axum::{
  extract::{Query, Request},
  http::{header, HeaderMap, HeaderValue, StatusCode, Uri},
  response::IntoResponse,
};
use base64::Engine;
use enum_map::enum_map;
use maud::{html, Markup, PreEscaped};
use once_cell::sync::Lazy;
use serde_variant::to_variant_name;

use crate::{
  render::{OGImage, OGTheme},
  server::{AppError, Res},
  utils::{safe_id, str_or_val},
};

static PUBLIC_URL: Lazy<String> = Lazy::new(|| {
  return std::env::var("PUBLIC_URL").unwrap_or("http://localhost:8080".to_string());
});

fn base(title: &str, node: Markup) -> Markup {
  let og_short = "OpenGraph";
  let og_title = "OpenGraph Preview & Generate Social Media Meta Tags";
  let og_descr = "OpenGraph lets you preview and generate Open Graph meta tags easily, ensuring your website content displays beautifully on social media.";

  let ogi_url = format!(
    "{}/v0/png?title={}&author={}&photo={}&url={}&theme={}",
    PUBLIC_URL.as_str(),
    urlencoding::encode(og_title),
    og_short,
    format!("{}{}", PUBLIC_URL.as_str(), "/assets/favicon.svg"),
    PUBLIC_URL.as_str().split("://").nth(1).unwrap(),
    to_variant_name(&OGTheme::Tinacious).unwrap()
  );

  html!({
    (maud::DOCTYPE)
    html {
      head {
        meta charset="utf-8" {}
        meta name="viewport" content="width=device-width, initial-scale=1" {}
        meta name="color-scheme" content="light dark" {}
        meta name="robots" content="index,follow" {}

        title { (title) }

        link rel="icon" type="image/svg+xml" href="/assets/favicon.svg" {}
        link rel="preconnect" href="https://unpkg.com" {}
        link rel="preconnect" href="https://fonts.googleapis.com" {}
        link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" {}
        link rel="preconnect" href="https://cloud.umami.is" {}

        script defer src="https://cloud.umami.is/script.js" data-website-id="185ce86b-a0f1-42a6-83ea-d9e0e502c9d6" {}

        link rel="stylesheet" href="https://unpkg.com/normalize.css" media="screen" {}
        link rel="stylesheet" href="https://unpkg.com/sakura.css/css/sakura.css" media="screen" {}
        link rel="stylesheet" href="https://unpkg.com/sakura.css/css/sakura-dark.css" media="screen and (prefers-color-scheme: dark)" {}
        link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@300..800&display=swap" {}
        link rel="stylesheet" href="/assets/app.css" {}
        script src="https://unpkg.com/htmx.org@2.0" {}
        script src="https://unpkg.com/@twind/cdn" {}

        link rel="stylesheet" href="https://unpkg.com/@highlightjs/cdn-assets@11.9.0/styles/default.min.css" {}
        script defer src="https://unpkg.com/@highlightjs/cdn-assets@11.9.0/highlight.min.js" {}

        meta name="description" content=(og_descr) {}
        meta property="og:url" content=(PreEscaped(PUBLIC_URL.as_str())) {}
        meta property="og:type" content="website" {}
        meta property="og:title" content=(og_title) {}
        meta property="og:description" content=(og_descr) {}
        meta property="og:image" content=(PreEscaped(ogi_url)) {}
        meta property="og:image:alt" content=(og_short) {}
        meta property="og:image:width" content="1200" {}
        meta property="og:image:height" content="630" {}
        meta property="og:site_name" content=(og_short) {}
        meta name="twitter:card" content="summary_large_image" {}
      }
      body {
        main { (node) }
        script src="/assets/app.js" {}
      }
    }
  })
}

fn form_input(label: &str, name: &str, value: &str) -> Markup {
  let for_id = safe_id();

  html! {
    div {
      label for=(for_id) { (label) }
      input .w-full id=(for_id) name=(name) value=(value) {}
    }
  }
}

fn form_select(label: &str, name: &str, options: &Vec<(&str, &str)>) -> Markup {
  let for_id = safe_id();

  html! {
    div {
      label for=(for_id) { (label) }
      select .w-full id=(for_id) name=(name) {
        @for (value, label) in options {
          option value=(value) { (label) }
        }
      }
    }
  }
}

// MARK: Handlers

async fn load_base64_image(url: &str) -> Res<String> {
  let ua = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

  let mut headers = HeaderMap::new();
  headers.insert("User-Agent", HeaderValue::from_str(&ua)?);

  let client = reqwest::Client::builder()
    .default_headers(headers)
    .read_timeout(std::time::Duration::from_secs(10))
    .build()?;

  let rep = client.get(url).send().await?;

  let max_size = 1024 * 1024 * 5; // 5MB
  match rep.content_length() {
    None => return AppError::new("Image size is unknown"),
    Some(len) if len > max_size => return AppError::new("Image is too large"),
    _ => {}
  };

  let allowed = ["image/jpeg", "image/png", "image/webp", "image/svg+xml"];
  let mime = match rep.headers().get(header::CONTENT_TYPE) {
    None => return AppError::new("Content type is unknown"),
    Some(ct) if !allowed.contains(&ct.to_str().unwrap()) => {
      return AppError::new(&format!("Content type is not allowed: {:?}", ct))
    }
    Some(ct) => ct.to_str().unwrap().to_string(),
  };

  let rep = rep.bytes().await?;
  let rep = base64::engine::general_purpose::STANDARD.encode(rep);
  let rep = format!("data:{mime};base64,{rep}");

  Ok(rep)
}

async fn get_ogi(uri: &Uri, load_photo: bool) -> Res<OGImage> {
  let ogi_default = OGImage::default();
  let mut ogi: Query<OGImage> = Query::try_from_uri(uri).unwrap();
  ogi.title = str_or_val(&ogi.title, &ogi_default.title);
  ogi.photo = str_or_val(&ogi.photo, &ogi_default.photo);
  ogi.author = str_or_val(&ogi.author, &ogi_default.author);
  ogi.url = str_or_val(&ogi.url, &ogi_default.url);

  if load_photo {
    ogi.photo = load_base64_image(&ogi.photo).await?;
  }

  Ok(ogi.0)
}

pub async fn ogi_svg(req: Request) -> Res<impl IntoResponse> {
  let ogi = get_ogi(req.uri(), false).await?;
  Ok((StatusCode::OK, [(header::CONTENT_TYPE, "image/svg+xml")], ogi.render_svg()))
}

pub async fn ogi_png(req: Request) -> Res<impl IntoResponse> {
  let ogi = get_ogi(req.uri(), true).await?;
  Ok((StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], ogi.render_png()))
}

pub async fn index() -> Res<impl IntoResponse> {
  let ogi = OGImage::default();

  let themes = enum_map! {
    OGTheme::Default => "Default",
    OGTheme::NightOwl => "Night Owl",
    OGTheme::Github => "GitHub",
    OGTheme::Matrix => "Maxtrix",
    OGTheme::Dracula => "Dracula",
    OGTheme::Tinacious => "Tinacious",
    OGTheme::ShadesOfPurple => "Shades of Purple",
  };

  let themes = themes.iter().map(|(v, l)| (to_variant_name(&v).unwrap(), *l)).collect::<Vec<_>>();

  let form = maud::html!({
    form hx-trigger="input delay:500ms from:input, input from:select" hx-get="/v0/svg" hx-target="#ogpi" {
      fieldset {
        (form_input("Title", "title", &ogi.title))
        (form_input("Author", "author", &ogi.author))
        (form_input("Image URL", "photo", &ogi.photo))
        (form_input("Website URL", "url", &ogi.url))
        (form_select("Theme", "theme", &themes))
      }
    }
  });

  let tokens = vec!["title", "author", "photo", "url", "theme"];
  let tokens = tokens
    .iter()
    .map(|x| format!("{x}=<span class=\"kw\">{{{x}}}</span>"))
    .collect::<Vec<_>>()
    .join("&");

  let public_url = format!("{}/v0/png?{}", PUBLIC_URL.as_str(), tokens);

  // let raw = include_str!("../content/seo.md");
  // let doc = load_md(raw);

  let html = html! {
    hgroup class="text-center mb-5" {
      h2 class="mt-5 mb-2 flex items-center justify-center gap-5" {
        // img src="/assets/favicon.svg" class="w-16 h-16 mb-0" {}
        "Open Graph Image Generator"
      }

      // div class="flex justify-center items-center gap-x-5" {
      //   "Generate Open Graph images for your static pages"
      //   a href="https://github.com/vladkens/ghstats" target="_blank" class="hover:border-none" {
      //     img src="https://badgen.net/github/stars/vladkens/ghstats" class="m-0" {}
      //   }
      // }
    }

    div class="flex-col flex md:flex-row gap-2" {
      div class="w-full md:w-1/2 flex flex-col items-center justify-center gap-1" {
        div #ogpi { (ogi.render_svg()) }
        div class="flex justify-center gap-5 mt-2" {
          button #open_url class="w-[150px]" { "Open" }
          button #copy_url class="w-[150px]" { "Copy" }
        }
      }
      div class="w-full md:w-1/2" { (form) }
    }

    h4 class="text-center" { "How to use?" }
    p { "Add the following meta tag to " code { "<head>" } " section of your HTML page and replace highlighted tokens:" }
    pre class="whitespace-pre-wrap" {
      "<meta property=\"og:image\" content=\"" (PreEscaped(public_url)) "\" />"
    }

    // (doc)
  };

  Ok(base("OpenGraph Image Generator", html))
}
