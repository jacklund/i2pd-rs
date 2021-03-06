use i2p::error::Error;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct HashedStorage {
    directory: PathBuf,
    hash_keys: bool,
}

trait Storage<T> {
    fn store(&self, key: &str, value: &T) -> Result<(), Error>;
    fn load(&self) -> Result<HashMap<String, T>, Error>;
}

impl HashedStorage {
    pub fn new(data_dir: &Path,
               app: &str,
               kind: &str,
               hash_keys: bool)
               -> Result<HashedStorage, Error> {
        let mut hashed_storage: HashedStorage = Default::default();
        hashed_storage.initialize(data_dir, app, kind, hash_keys)?;

        Ok(hashed_storage)
    }

    fn initialize(&mut self,
                  data_dir: &Path,
                  app: &str,
                  kind: &str,
                  hash_keys: bool)
                  -> Result<(), Error> {
        self.directory = self.create_storage_directory(data_dir, app, kind)?;
        self.hash_keys = hash_keys;

        Ok(())
    }

    fn create_storage_directory(&self,
                                data_dir: &Path,
                                app: &str,
                                kind: &str)
                                -> Result<PathBuf, Error> {
        let mut dir = PathBuf::new();
        dir.push(data_dir);
        dir.push(app);
        dir.push(kind);
        info!("Creating storage directory {:?}", dir);
        if !dir.exists() {
            fs::create_dir_all(dir.to_owned())?;
        } else if !dir.is_dir() {
            return Err(Error::Configuration(format!("Path {} exists but is not a directory",
                                           dir.to_str().unwrap())));
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
        file_path.push(self.directory.to_owned());
        file_path.push(filename.chars().nth(0).unwrap().to_string());
        file_path.push(filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(fs::File::create(file_path)?)
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = Vec::new();
        fs::File::open(path)?.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    pub fn remove(&self, key: &str) {
        unimplemented!()
    }
}

impl<T> Storage<T> for HashedStorage {
    fn store(&self, key: &str, value: &T) -> Result<(), Error> {
        let mut file: fs::File = self.create_file(&self.get_filename(key)?)?;
        file.sync_all()?;

        Ok(())
    }

    fn load(&self) -> Result<HashMap<String, T>, Error> {
        let mut map: HashMap<String, T> = HashMap::new();
        for result in WalkDir::new(self.directory.to_owned()) {
            match result {
                Ok(entry) => {
                    if entry.path().is_file() {
                        unimplemented!()
                        // map.insert(entry.file_name().to_str().unwrap().to_string(),
                        //            &self.read(entry.path())?[..]);
                    }
                }
                Err(error) => return Err(Error::from(io::Error::from(error))),
            }
        }

        Ok(map)
    }
}

#[cfg(test)]
mod test {
    use i2p::data::router_info::{RouterAddress, SupportedTransports};
    use i2p::fs::hashed_storage::{HashedStorage, Storage};
    use std::collections::HashMap;
    use std::env;
    use tempdir::TempDir;

    #[test]
    fn test_store_and_load() {
        let data_dir = TempDir::new("i2pd-test").unwrap();
        let mut hashed_storage = HashedStorage::new(data_dir.path(), "test", "router-info", false)
            .unwrap();
        // let mut address: RouterAddress = Default::default();
        // address.cost = 100;
        // address.expiration = None;
        // address.transport_style = SupportedTransports::SSUV4;
        // hashed_storage.store("foo", &address);
        // let data: HashMap<String, RouterAddress> = hashed_storage.load().unwrap();
        // assert_eq!(address, *data.get("foo").unwrap());
    }
}