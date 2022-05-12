use crate::hooks::use_storage;

pub fn is_dark() -> bool {
    let storage = use_storage();
    if storage.is_err() {
        // default mode: light
        return false;
    }
    let storage = storage.unwrap();

    let current_mode = storage.get_item("mode").unwrap_or(None);
    if current_mode.is_none() {
        false
    } else {
        current_mode.unwrap().to_lowercase() == "dark"
    }
}

pub fn mode(dark: bool) {
    let storage = use_storage();
    if storage.is_err() {
        return ;
    }
    let storage = storage.unwrap();
    let _ = storage.set_item("mode", if dark { "dark" } else { "light" });
    if dark {
        let _ = js_sys::eval("document.documentElement.classList.add('dark');");
    } else {
        let _ = js_sys::eval("document.documentElement.classList.remove('dark');");
    }
}