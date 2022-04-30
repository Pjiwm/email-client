use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
pub struct FileManager {
    name: String,
}
impl FileManager {
    pub fn new(file_name: &str) -> Self {
        FileManager {
            name: file_name.to_owned(),
        }
    }
    pub fn read_file(&self) -> String {
        let file = File::open(format!("data/{}", &self.name));
        let file: File = match file {
            Ok(f) => f,
            Err(_) => create_file(&self.name),
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).ok();
        contents
    }

    pub fn write_file(&self, new_line: &str) {
        let file = File::options()
            .append(true)
            .open(format!("data/{}", &self.name));

        let mut file = match file {
            Ok(f) => f,
            Err(_) => {
                create_file(&self.name);
                File::options()
                    .append(true)
                    .open(format!("data/{}", &self.name))
                    .ok()
                    .unwrap()
            }
        };

        file.write_all(format!("{}\n", new_line).as_bytes())
            .expect("An error occured when syncing the file");
        file.sync_all()
            .expect(format!("{}", "An error occured when syncing the file").as_str());
    }

    #[allow(dead_code)]
    pub fn remove_file(&self) -> std::io::Result<()> {
        fs::remove_file(format!("data/{}", &self.name))?;
        Ok(())
    }
}

pub fn create_file(file_name: &str) -> File {
    println!("file, {} was created", file_name);
    let file = File::create(format!("data/{}", file_name)).ok();
    return file.expect("Failed to create file");
}
