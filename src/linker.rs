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
    pub skip_libs: Vec<String>,
}

pub struct LinkerBuilder {
    exec: PathBuf,
    qt: Option<PathBuf>,
    skip_libs: Vec<String>,
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

    fn list_objects(
        objects: &mut HashMap<String, PathBuf>,
        path: impl AsRef<Path>,
        skip_libs: &[String],
    ) {
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
            if parts.len() == 4 && !skip_libs.iter().any(|lib| parts[0].starts_with(lib)) {
                objects.insert(parts[0].to_string(), parts[2].into());
                Self::list_objects(objects, parts[2], skip_libs)
            }
        }
    }

    pub fn new(exec: impl AsRef<Path>) -> Self {
        let exec = fs::canonicalize(exec.as_ref()).expect("Wrong exec path");
        Self {
            exec,
            qt: None,
            skip_libs: vec![],
        }
    }

    pub fn with_skip_libs(mut self, skip_libs: Vec<String>) -> Self {
        self.skip_libs = skip_libs;
        self
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
        Self::list_objects(&mut objects, &self.exec, &self.skip_libs);
        if let Some(qt) = &self.qt {
            for entry in WalkDir::new(qt) {
                match entry {
                    Ok(e) if e.file_type().is_file() => {
                        Self::list_objects(&mut objects, e.path(), &self.skip_libs);
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
            skip_libs: self.skip_libs.clone(),
        }
    }
}
