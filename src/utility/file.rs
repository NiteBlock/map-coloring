use std::path::Path;

pub fn get_next_file_path() -> String {
    let mut i = 1;
    while Path::new(&format!("./maps/{}.toml", i)).exists() {
        i += 1;
    }
    return format!("./maps/{}.toml", i);
}
