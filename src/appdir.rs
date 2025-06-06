use std::fs;
use std::os::unix::fs::PermissionsExt;
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
    bundle: bool,
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
            bundle: false,
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

    pub fn with_bundle(mut self) -> Self {
        self.bundle = true;
        self
    }

    fn bundle(&self) {
        let name = self
            .linker
            .exec
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let run_path = self.path.join("run.sh");
        let run_content = format!(
            "#!/bin/sh\nDIR=$(dirname \"$0\")\nexport LD_LIBRARY_PATH=\"$DIR/lib\"\nexec \"$DIR/{}\" \"$@\"\n",
            name
        );

        fs::write(&run_path, run_content).expect("Failed to write run file");

        let mut perms = fs::metadata(&run_path)
            .expect("Failed to get metadata")
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&run_path, perms).expect("Failed to set permissions");

        Command::new("makeself")
            .current_dir(self.path.parent().unwrap())
            .arg("--notemp")
            .arg(self.path.clone())
            .arg(format!("{}.run", name))
            .arg(format!("{} App", name))
            .arg("./run.sh")
            .status()
            .expect("failed to run patchelf")
            .success()
            .then_some(())
            .expect("patchelf failed");

        fs::remove_dir_all(self.path.clone()).expect("Failed to remove directory");
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
        if self.bundle {
            self.bundle();
        }
        AppDir {
            path: self.path.clone(),
        }
    }
}
