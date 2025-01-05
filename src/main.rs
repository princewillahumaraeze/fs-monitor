use std::collections::HashMap;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::env;

#[derive(Debug, PartialEq)]
enum FileState {
    Modified,
    Created,
    Deleted,
}

#[derive(Debug)]
struct FileMonitor {
    directory: PathBuf,
    file_states: HashMap<PathBuf, SystemTime>,
}

impl FileMonitor {
    fn new(directory: &str) -> Self {
        FileMonitor{
            directory: PathBuf::from(directory),
            file_states: HashMap::new(),
        }
    }

    fn scan_directory(&self) -> HashMap<PathBuf, SystemTime>{
        let mut file_states = HashMap::new();
        if let Ok(entries) = fs::read_dir(&self.directory){
            for entry in entries.flatten(){
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file(){
                       if let Ok(modified_time) = metadata.modified(){
                           file_states.insert(entry.path(), modified_time);
                       }
                    }
                }
            }
        }
        file_states
    }

    fn detect_changes(&mut self,) -> Vec<(PathBuf, FileState)>{
        let mut changes = Vec::new();
        let current_file_states = self.scan_directory();

        // Detect File Creations and Modifications
        for (path, modified_time) in &current_file_states{
            match self.file_states.get(path){
                Some(prev_time) => {
                    if modified_time > prev_time{
                        changes.push((path.clone(), FileState::Modified));
                    }
                }
                None => changes.push((path.clone(), FileState::Created)),
            }
        }
        self.file_states = current_file_states;
        changes
    }

}


fn main() {


    let direct_to_monitor = "./monitor_dir"; // Change to your directory
    let mut monitor = FileMonitor::new(direct_to_monitor);

    println!("Monitoring Directory: {}", direct_to_monitor);
    loop {
        let changes  = monitor.detect_changes();
        for (path, state) in changes {
            match state {
                FileState::Modified => println!("File Modified: {:?}", path),
                FileState::Created => println!("File Created: {:?}", path),
                FileState::Deleted => println!("File Deleted: {:?}", path),
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}