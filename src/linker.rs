use super::SKIP_LIBS;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Linker {
    pub exec: PathBuf,
    pub objects: Vec<PathBuf>,
    pub qt: Option<PathBuf>,
}

pub struct LinkerBuilder {
    exec: PathBuf,
    qt: Option<PathBuf>,
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
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 && !SKIP_LIBS.iter().any(|lib| parts[0].starts_with(lib)) {
                objects.insert(parts[0].to_string(), parts[2].into());
                Self::list_objects(objects, parts[2])
            }
        }
    }

    pub fn new(exec: impl AsRef<Path>) -> Self {
        let exec = fs::canonicalize(exec.as_ref()).expect("Wrong exec path");
        Self { exec, qt: None }
    }

    pub fn with_qt(mut self, path: impl AsRef<Path>) -> Self {
        let path = fs::canonicalize(path.as_ref())
            .expect("Wrong Qt path")
            .join("plugins");

        self.qt = Some(path.to_path_buf());
        self
    }

    pub fn build(&self) -> Linker {
        let mut objects = HashMap::new();
        Self::list_objects(&mut objects, &self.exec);
        if let Some(qt) = &self.qt {
            for entry in WalkDir::new(qt) {
                match entry {
                    Ok(e) if e.file_type().is_file() => {
                        Self::list_objects(&mut objects, e.path());
                    }
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
        Linker {
            exec: self.exec.clone(),
            objects: objects
                .values()
                .filter(|s| s.to_str() != Some("not"))
                .cloned()
                .collect(),
            qt: self.qt.clone(),
        }
    }
}
