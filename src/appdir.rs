use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

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

    fn patch_qt(&self) {
        for entry in WalkDir::new(self.path.join("plugins")) {
            match entry {
                Ok(e) if e.file_type().is_file() => {
                    Command::new("patchelf")
                        .arg("--set-rpath")
                        .arg("$ORIGIN/../../lib/")
                        .arg(e.path())
                        .status()
                        .expect("failed to run patchelf")
                        .success()
                        .then_some(())
                        .expect("patchelf failed");
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }

        let qt_conf_path = self.path.join("qt.conf");
        let qt_conf_content = "[Paths]\nPlugins = plugins\n";

        fs::write(&qt_conf_path, qt_conf_content).expect("Failed to write qt.conf file");
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

    fn patch_files(&self) {
        Command::new("patchelf")
            .arg("--set-rpath")
            .arg("$ORIGIN/lib")
            .arg(&self.linker.exec)
            .status()
            .expect("failed to run patchelf")
            .success()
            .then_some(())
            .expect("patchelf failed");
        self.linker.objects.iter().for_each(|i| {
            let destination = self.path.join("lib").join(i.file_name().unwrap());
            Command::new("patchelf")
                .arg("--set-rpath")
                .arg("$ORIGIN")
                .arg(destination)
                .status()
                .expect("failed to run patchelf")
                .success()
                .then_some(())
                .expect("patchelf failed");
        });
    }

    pub fn build(&self) -> AppDir {
        fs::create_dir(self.path.clone()).expect("Cannot create appdir");
        fs::create_dir(self.path.join("lib")).expect("Cannot create appdir");
        if let Some(qt) = &self.linker.qt {
            Command::new("cp")
                .arg("-r")
                .arg(qt)
                .arg(self.path.clone())
                .status()
                .expect("Cannot copy qt dir");
            self.patch_qt();
        }
        self.copy_exec();
        self.copy_files();
        self.patch_files();
        AppDir {
            path: self.path.clone(),
        }
    }
}
