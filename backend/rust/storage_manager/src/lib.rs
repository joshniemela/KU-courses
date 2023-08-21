//! # Storage handler
//! Library to handle simple interactions with storage in a location agnostic manner.
use std::fs;

trait Config {}

trait Storage<C, T> where C: Config {
    fn new(conf: C) -> Result<(), Box<dyn std::error::Error>>;
    fn read(filepath: &str) -> Result<T, Box<dyn std::error::Error>>;
    fn write(filepath: &str, input: T) -> Result<(), Box<dyn std::error::Error>>;
    fn delete(filepath: &str) -> Result<(), Box<dyn std::error::Error>>;
}


struct LocalStorage {
    config: LocalStorageConfig
}
struct LocalStorageConfig{
    root: String
}

impl LocalStorageConfig {
    fn new(root: String) -> LocalStorageConfig {
        LocalStorageConfig { root }
    }
}

impl Config for LocalStorageConfig{}

impl Storage<LocalStorageConfig, Box<[u8]>> for LocalStorage {
    fn new(conf: LocalStorageConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Check read / write / delete permissions
        let md = fs::metadata(&conf.root)?;
        let permissions = md.permissions();
        let readonly = permissions.readonly();
        if readonly {
            return Err(format!("No write permission to {}, storage implementation needs write permission", &conf.root).into());
        }
        Ok(())
    }

    fn read(filepath: &str) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
        unimplemented!() // Implement the actual read logic here
    }

    fn write(filepath: &str, input: Box<[u8]>) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!() // Implement the actual write logic here
    }

    fn delete(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn no_such_dir() {
        let conf = LocalStorageConfig::new(String::from("foo"));
        let storage = LocalStorage::new(conf).unwrap();
    }

    #[test]
    fn valid_dir() {
        let dir: String = String::from(std::env::current_exe().unwrap().as_path().to_string_lossy());
        println!("{}", dir);
        let conf = LocalStorageConfig::new(dir);
        let storage = LocalStorage::new(conf).unwrap();
    }

    #[test]
    #[ignore] // Ignored due to potential issues on windows, useful for when testing on linux
    fn valid_dir_home_user() {
        let dir: String = String::from(std::env::home_dir().unwrap().as_path().to_string_lossy());
        println!("{}", dir);
        let conf = LocalStorageConfig::new(dir);
        let storage = LocalStorage::new(conf).unwrap();
    }
}
