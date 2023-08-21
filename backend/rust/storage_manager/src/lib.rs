//! # Storage handler
//! Library to handle simple interactions with storage in a location agnostic manner.
use std::{fs, io::{Read, Write}};

/// Generic trait for interacting with file-system like storage.
///
/// This trait is meant to provide an agnostic way to interact with file storage,
/// wether locally or on ex. S3. Any storage solution implementing this trait,
/// will more or less act like a native file system.
/// # Current implementations:
/// * `LocalStorage` - Simple implementation for the local filesystem
/// # Examples
/// ## Creating and using local storage
/// ```
/// use storage_manager::{LocalStorage, LocalStorageConfig, Storage};
///
/// // Long way of saying "home directory" which should be valid for linux users!
/// let root: String = String::from(std::env::home_dir().unwrap().as_path().to_string_lossy());
/// let config = LocalStorageConfig{ root };
/// let storage = LocalStorage::new(config).expect("unable to create storage!");
/// 
/// // Some example data we want to write
/// let data = String::from("Hello world!");
///
/// storage.write("test.txt", &data).expect("Error writing file");
/// storage.write("nested_dir/nested_file.txt", &data).expect("Error writing nested file");
///
/// let res = storage.read("test.txt").expect("Error reading file!");
/// println!("{}", &res);
/// let nested_res = storage.read("nested_dir/nested_file.txt").expect("Error reading nested
/// file!");
/// println!("{}", &nested_res);
///
/// storage.delete("test.txt").expect("Error deleting file");
/// storage.delete("nested_dir/nested_file.txt").expect("Error deleting nested file");
/// ```
pub trait Storage {
    type Args;

    /// Used to create new instances of Storage.
    ///
    /// # Parameters
    /// * `args: Self::Args` - This is generic in order to provide each implementation the ability
    /// to define which arguments are nescessary when initializnig that specific implementation.
    /// Generally Args will follow the convention of adding *Config at the end of the type of
    /// storage.
    /// For example `LocalStorage` will set its `Self::Args` to equal `LocalStorageConfig`.
    ///
    /// # Returns
    /// This function either returns an instance of `Self` (meaning that it was succesfully
    /// initialized) or _some_ error (which depends on the specific implementation. For example,
    /// errors from the local storage implementation will like be `std::io` related.
    fn new(args: Self::Args) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;

    /// Used to read a file from storage.
    ///
    /// # Parameters
    /// * `&self` - Reference to itself, in order to get at possible values defined on the struct.
    /// * `filepath: &str` - The filepath (relative to the storage root) including the filename.
    /// 
    /// # Returns
    /// Either returns a `String` with the result or _some_ error depending on the implementation.
    fn read(&self, filepath: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// Used to write a file to storage.
    ///
    /// # Parameters
    /// * `&self` - Reference to itself, in order to get at possible values defined on the struct.
    /// * `filepath: &str` - The filepath (relative to the storage root) including the filename.
    /// * `input: &String` - The string contents to be written.
    /// 
    /// # Returns
    /// Either returns either Ok or the error incurred.
    fn write(&self, filepath: &str, input: &String) -> Result<(), Box<dyn std::error::Error>>;

    /// Used to delete a file from storage.
    ///
    /// # Parameters
    /// * `&self` - Reference to itself, in order to get at possible values defined on the struct.
    /// * `filepath: &str` - The filepath (relative to the storage root) including the filename.
    ///
    /// # Returns
    /// Either returns Ok or the error incurred.
    fn delete(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>>;
}


/// The barebones struct dictating the local storage.
///
/// More fields could be added here in the future if needed.
///
/// # Fields
/// * `config` - An instance of `LocalStorageConfig` (which is used when creating a new instance of
/// `LocalStorage`
pub struct LocalStorage {
    config: LocalStorageConfig
}

/// Struct dictating the configuration of local storage.
///
/// This struct mirrors the information needed in `LocalStorage`, and is meant to provide a place
/// to configure your local storage before instantiating it.
///
/// # Fields
/// * `pub root: String`: The absolute path to the directory you'd want as your entry point for
/// your application. Every "filepath" in read / write / delete will be relative to this absolute
/// path.
pub struct LocalStorageConfig {
    pub root: String
}

impl LocalStorage {
    /// Simple function that constructs the absolute path using the root path on the LocalStorage
    /// and the new file path.
    fn construct_filepath(&self, new_file_path: &str) -> String {
        let r = self.config.root.as_str();
        let fp = format!("{}/{}", r, new_file_path);
        fp
    }
}

impl Storage for LocalStorage {
    type Args = LocalStorageConfig;
    /// Read implementation for LocalStorage.
    ///
    /// # Parameters
    /// * `args: Self::Args` - When creating an instance of LocalStorage, the Args type from the
    /// storage interface is set to `LocalStorageConfig`.
    fn new(args: Self::Args) -> Result<Self, Box<dyn std::error::Error>> {
        // Check read / write / delete permissions
        let md = fs::metadata(&args.root)?;
        let permissions = md.permissions();
        let readonly = permissions.readonly();
        if readonly {
            return Err(format!("No write permission to {}, storage implementation needs write permission", &args.root).into());
        }
        Ok(LocalStorage{
            config: args
        })
    }
    
    // Read implementation for local storage
    fn read(&self, filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut data = String::new();
        let mut file = fs::File::open(self.construct_filepath(filepath))?;
        file.read_to_string(&mut data)?;
        Ok(data)
    }
    
    // Write implementation for local storage
    fn write(&self, filepath: &str, input: &String) -> Result<(), Box<dyn std::error::Error>> {
        let full_filepath = self.construct_filepath(filepath);

        // extract the directory path from the full filepath
        let directory_path = std::path::Path::new(&full_filepath)
            .parent()
            .ok_or("Invalid filepath")?;

        // Create the directory hierarchy if it doesn't exist
        if !directory_path.exists() {
            fs::create_dir_all(directory_path)?;
        }

        let mut file = fs::File::create(self.construct_filepath(filepath))?;
        file.write_all(input.as_bytes())?;
        Ok(())
    }
    
    // Delete implementation for local storage
    fn delete(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::remove_file(self.construct_filepath(filepath))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn no_such_dir() {
        let root = String::from("foo");
        let conf = LocalStorageConfig{ root };
        let _storage = LocalStorage::new(conf).unwrap();
    }

    #[test]
    fn valid_dir() {
        let root: String = String::from(std::env::current_exe().unwrap().as_path().to_string_lossy());
        println!("{}", root);
        let conf = LocalStorageConfig{ root };
        let _storage = LocalStorage::new(conf).unwrap();
    }

    #[test]
    fn write_file() {
        let root: String = String::from(std::env::current_exe().unwrap().parent().unwrap().to_string_lossy());
        println!("{}", root);
        let conf = LocalStorageConfig{ root };
        let storage = LocalStorage::new(conf).unwrap();
        let data = String::from("Hello world! 2.0");
        storage.write("test.txt", &data).unwrap();
    }

    #[test]
    fn write_file_recursively() {
        let root: String = String::from(std::env::current_exe().unwrap().parent().unwrap().to_string_lossy());
        println!("{}", root);
        let conf = LocalStorageConfig{ root };
        let storage = LocalStorage::new(conf).unwrap();
        let data = String::from("Hello world! 2.0");
        storage.write("new_dir/test.txt", &data).unwrap();

        // Read in the file
        storage.read("new_dir/test.txt").unwrap();
    }

    #[test]
    fn write_file_recursively_deep() {
        let root: String = String::from(std::env::current_exe().unwrap().parent().unwrap().to_string_lossy());
        println!("{}", root);
        let conf = LocalStorageConfig{ root };
        let storage = LocalStorage::new(conf).unwrap();
        let data = String::from("Hello world! 2.0");
        storage.write("new_dir/new_dir_two/new_dir_three/test.txt", &data).unwrap();

        // Read in the file
        storage.read("new_dir/new_dir_two/new_dir_three/test.txt").unwrap();
    }

    #[test]
    fn read_file() {
        let root: String = String::from(std::env::current_exe().unwrap().parent().unwrap().to_string_lossy());
        println!("{}", root);
        let conf = LocalStorageConfig{ root };
        let storage = LocalStorage::new(conf).unwrap();
        
        // Write a file
        let data = String::from("Hello world! 2.0");
        storage.write("test.txt", &data).unwrap();

        // Read the file
        let read_file = storage.read("test.txt").unwrap();
        println!("{}", read_file);
    }

    #[test]
    #[should_panic]
    fn delete_file() {
        let root: String = String::from(std::env::current_exe().unwrap().as_path().to_string_lossy());
        println!("{}", root);
        let conf = LocalStorageConfig{ root };
        let storage = LocalStorage::new(conf).unwrap();
        
        // Write a file
        let data = String::from("Hello world! 2.0");
        storage.write("test.txt", &data).unwrap();

        // delete the file
        storage.delete("text.txt").unwrap();

        // Read the file
        let read_file = storage.read("test.txt").unwrap();
        println!("{}", read_file);
    }
}
