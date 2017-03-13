use i2p::error::Error;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct HashedStorage {
    directory: PathBuf,
    hash_keys: bool,
}

impl HashedStorage {
    pub fn new(data_dir: &PathBuf, app: &str, kind: &str, hash_keys: bool) -> Result<HashedStorage, Error> {
        let mut hashed_storage: HashedStorage = Default::default();
        hashed_storage.initialize(data_dir, app, kind, hash_keys);

        Ok(hashed_storage)
    }

    fn initialize(&mut self, data_dir: &PathBuf, app: &str, kind: &str, hash_keys: bool) -> Result<(), Error> {
        self.directory = self.create_storage_directory(data_dir, app, kind)?;
        self.hash_keys = hash_keys;

        Ok(())
    }

    fn create_storage_directory(&self, data_dir: &PathBuf, app: &str, kind: &str) -> Result<PathBuf, Error> {
        let mut dir = PathBuf::new();
        dir.push(data_dir);
        dir.push(app);
        dir.push(kind);
        if !dir.exists() {
            fs::create_dir_all(dir.to_owned());
        } else if !dir.is_dir() {
            return Err(Error::Configuration(format!("Path {} exists but is not a directory", dir.to_str().unwrap())));
        }

        Ok(dir)
    }

    fn get_filename(&self, key: &str) -> Result<String, Error> {
        if self.hash_keys {
            unimplemented!()
        } else {
            Ok(key.to_owned())
        }
    }

    fn create_file(&self, filename: &str) -> Result<fs::File, Error> {
        let mut file_path = PathBuf::new();
        file_path.push(filename.chars().nth(0).unwrap().to_string());
        file_path.push(filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent);
        }
        Ok(fs::File::create(file_path)?)
    }

    pub fn store(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        let mut file: fs::File = self.create_file(&self.get_filename(key)?)?;
        file.write_all(value)?;

        Ok(())
    }

    pub fn load(&self) -> Result<HashMap<String, Vec<u8>>, Error> {
        unimplemented!()
    }

    pub fn remove(&self, key: &str) {
        unimplemented!()
    }
}