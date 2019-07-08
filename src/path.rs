// Functions for dealing with the shell's executable path
use std::collections::HashMap;
use std::fs::read_dir;
use std::fs::DirEntry;

pub struct Cache {
    map: HashMap<String, String>,
}

impl Cache {
    fn add_entry(&mut self, entry: DirEntry) {
        let file_name = entry.file_name().into_string().unwrap();
        let file_path = entry.path().into_os_string().into_string().unwrap();
        self.map.insert(file_name, file_path);
    }

    pub fn new() -> Cache {
        let mut cache = Cache {
            map: HashMap::new(),
        };

        let path = std::env::var("PATH").unwrap();
        for segment in path.split(':') {
            match read_dir(segment) {
                Ok(entries) => {
                    for entry in entries {
                        cache.add_entry(entry.unwrap());
                    }
                }
                Err(_) => (),
            }
        }

        cache
    }

    pub fn has(&self, name: String) -> bool {
        self.map.get(&name).is_some()
    }

    pub fn get_path(&self, name: String) -> &String {
        self.map.get(&name).ok_or(String::new()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has() {
        let mut cache = Cache::new();
        assert_eq!(cache.has(String::from("foo")), false);
        cache.map.insert(String::from("foo"), String::from("bar"));
        assert_eq!(cache.has(String::from("foo")), true);
    }
}
