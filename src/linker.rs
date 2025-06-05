use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct Linker {
    exec: PathBuf,
    objects: Vec<PathBuf>,
}

pub struct LinkerBuilder {
    exec: PathBuf,
}

impl LinkerBuilder {
    fn check_path(path: impl AsRef<Path>) -> Option<(String, PathBuf)> {
        let path = path.as_ref();

        if !path.exists() {
            return None;
        }

        let filename = path.file_name()?.to_str()?.to_string();
        Some((filename, path.to_path_buf()))
    }

    fn list_objects(objects: &mut HashMap<String, PathBuf>, path: impl AsRef<Path>) {
        let Some((filename, path)) = Self::check_path(path) else {
            return;
        };
        if objects.contains_key(&filename) {
            return;
        }

        let output = Command::new("ldd")
            .arg(path)
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            // Examples:
            // "libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f3e52a70000)"
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() == 4 {
                objects.insert(parts[0].to_string(), parts[2].into());
                Self::list_objects(objects, parts[1])
            }
        }
    }

    pub fn new(exec: impl AsRef<Path>) -> Self {
        Self {
            exec: exec.as_ref().to_path_buf(),
        }
    }

    pub fn build(&self) -> Linker {
        let mut objects = HashMap::new();
        Self::list_objects(&mut objects, self.exec.clone());
        Linker {
            exec: self.exec.clone(),
            objects: objects.values().cloned().collect(),
        }
    }
}
