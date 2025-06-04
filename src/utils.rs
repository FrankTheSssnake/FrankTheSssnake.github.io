use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub fn open_link(url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        if let Some(w) = window() {
            let _ = w.open_with_url(url);
        }
    }
}

pub fn choose_random_link<'a>(links: &'a [&'a str]) -> &'a str {
    if links.is_empty() {
        return "";
    }
    let index = gen_range(0, links.len() as i32) as usize;
    links[index]
}
