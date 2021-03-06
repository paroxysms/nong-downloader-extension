use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

pub struct State {
    pub link_ext: String,
    pub link_type: String,
    pub quality: String,
    pub song_id: String,
    pub auto_id: bool,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {
        link_ext: "ZmsdIQuywaE".to_string(),
        link_type: "YouTube".to_string(),
        quality: "128k".to_string(),
        song_id: "1085360".to_string(),
        auto_id: false,
    });
}

impl State {
    pub fn get<'a>() -> MutexGuard<'a, Self> {
        STATE.lock().unwrap()
    }
}
