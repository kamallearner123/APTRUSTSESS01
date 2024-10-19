pub struct idsm_event_filter {
    ids_sec_event_map_len : u8
}

const version:&str = "1.1.";

pub fn get_version() ->&'static str {
    version
}

pub trait ids_filter {
    fn do_filter(&self);
}

impl ids_filter for idsm_event_filter {
    fn do_filter(&self) {
        println!("In do filter");
    }
}
