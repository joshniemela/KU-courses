//! # Storage handler
//! Library to handle simple interactions with storage in a location agnostic manner.
use std::{fs, path::Path, io::{Read, Write}};

trait Config {}

trait Storage<C, T> where C: Config {
    fn new(&mut self, conf: C) -> Result<(), Box<dyn std::error::Error>>;
    fn read(&self, filepath: &str) -> Result<T, Box<dyn std::error::Error>>;
    fn write(&self, filepath: &str, input: T) -> Result<(), Box<dyn std::error::Error>>;
    fn delete(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>>;
}


struct LocalStorage {
    config: Option<LocalStorageConfig>
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

impl Storage<LocalStorageConfig, String> for LocalStorage {
    fn new(&mut self, conf: LocalStorageConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Check read / write / delete permissions
        let md = fs::metadata(&conf.root)?;
        let permissions = md.permissions();
        let readonly = permissions.readonly();
        if readonly {
            return Err(format!("No write permission to {}, storage implementation needs write permission", &conf.root).into());
        }
        self.config = Some(conf);
        Ok(())
    }

    fn read(&self, filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
        let binding = self.config.as_ref().expect("No config on storage!");
        let r = binding.root.as_str();
        let fp = format!("{}/{}", r, filepath);
        let mut data = String::new();
        let mut file = fs::File::open(fp)?;
        file.read_to_string(&mut data)?;
        Ok(data)
    }

    fn write(&self, filepath: &str, input: String) -> Result<(), Box<dyn std::error::Error>> {
        let binding = self.config.as_ref().expect("No config on storage!");
        let r = binding.root.as_str();
        let fp = format!("{}/{}", r, filepath);
        println!("{}", &fp);

        let mut file = fs::File::create(fp)?;
        file.write_all(input.as_bytes())?;
        Ok(())
    }

    fn delete(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        let mut storage = LocalStorage{ config: None };
        storage.new(conf).unwrap();
    }

    #[test]
    fn valid_dir() {
        let dir: String = String::from(std::env::current_exe().unwrap().as_path().to_string_lossy());
        println!("{}", dir);
        let conf = LocalStorageConfig::new(dir);
        let mut storage = LocalStorage{ config: None };
        storage.new(conf).unwrap();
    }

    #[test]
    #[ignore] // Ignored due to potential issues on windows, useful for when testing on linux
    fn valid_dir_home_user() {
        let dir: String = String::from(std::env::home_dir().unwrap().as_path().to_string_lossy());
        println!("{}", dir);
        let conf = LocalStorageConfig::new(dir);
        let mut storage = LocalStorage{ config: None };
        storage.new(conf).unwrap();
    }

    #[test]
    fn write_file() {
        let dir: String = String::from(std::env::current_exe().unwrap().as_path().parent().unwrap().to_string_lossy());
        println!("{}", dir);
        let conf = LocalStorageConfig::new(dir);
        let mut storage = LocalStorage{ config: None };
        storage.new(conf).unwrap();

        let data = String::from("Hello world! 2.0");
        storage.write("test.txt", data).unwrap();
    }

}
