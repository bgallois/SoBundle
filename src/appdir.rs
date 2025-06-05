use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct AppDir {
    path: PathBuf,
}

pub struct AppDirBuilder {
    path: PathBuf,
    linker: super::Linker,
}

impl AppDirBuilder {
    pub fn new(linker: super::Linker) -> Self {
        let path = linker
            .exec
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("appdir");
        Self {
            path: path.to_path_buf(),
            linker,
        }
    }

    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().join("appdir");

        if !path.exists() {
            panic!("Wrong appdir path!")
        }
        self.path = path.to_path_buf();
        self
    }

    fn copy_files(&self) {
        self.linker.objects.iter().for_each(|i| {
            let destination = self.path.join("lib").join(i.file_name().unwrap());
            fs::copy(i, destination).unwrap();
        });
    }

    fn copy_exec(&self) {
        let destination = self.path.join(self.linker.exec.file_name().unwrap());
        fs::copy(self.linker.exec.clone(), destination).unwrap();
    }
    fn patch_exec(&self) {
        let output = Command::new("patchelf")
            .arg("--set-rpath")
            .arg(self.path.join("lib"))
            .arg(self.linker.exec.clone())
            .output()
            .expect("failed to execute patchelf");
    }

    pub fn build(&self) -> AppDir {
        println!("{:?}", self.path);
        fs::create_dir(self.path.clone()).expect("Cannot create appdir");
        fs::create_dir(self.path.join("lib")).expect("Cannot create appdir");
        self.copy_exec();
        self.copy_files();
        self.patch_exec();
        AppDir {
            path: self.path.clone(),
        }
    }
}
