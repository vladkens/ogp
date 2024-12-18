use axum::extract::Request;
use nanoid::nanoid;

pub fn get_header<'a>(req: &'a Request, name: &'a str) -> Option<&'a str> {
  match req.headers().get(name) {
    Some(x) => Some(x.to_str().unwrap_or_default()),
    None => None,
  }
}

pub fn get_hx_target(req: &Request) -> Option<&str> {
  get_header(req, "hx-target")
}

pub fn safe_id() -> String {
  return nanoid!(10, &nanoid::alphabet::SAFE);
}

pub fn str_or_val(s: &str, default: &str) -> String {
  match s.trim() {
    "" => return default.to_string(),
    _ => s.trim().to_string(),
  }
}
