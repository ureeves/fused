use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

pub struct Build {
    out_dir: Option<PathBuf>,
}

pub struct Artifacts {
    pub lib_dir: PathBuf,
    pub includes: Vec<PathBuf>,
}

impl Build {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Build {
        Build {
            out_dir: env::var_os("OUT_DIR")
                .map(|s| PathBuf::from(s).join("libfuse")),
        }
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Build {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    fn cmd_meson(&self) -> Command {
        Command::new("meson")
    }

    fn cmd_ninja(&self) -> Command {
        Command::new("ninja")
    }

    pub fn build(&mut self) -> Artifacts {
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");

        let build_dir = out_dir.join("build");
        if !build_dir.exists() {
            fs::create_dir_all(&build_dir).unwrap();
        }

        let mut meson = self.cmd_meson();
        meson.arg("setup").arg(source_dir()).current_dir(&build_dir);
        self.run_command(meson, "Setting up meson");

        let mut ninja = self.cmd_ninja();
        ninja.current_dir(&build_dir);
        self.run_command(ninja, "Building with ninja");

        let include_dir = source_dir().join("include");

        let includes = vec![
            include_dir.join("cuse_lowlevel.h"),
            include_dir.join("fuse.h"),
            include_dir.join("fuse_common.h"),
            include_dir.join("fuse_kernel.h"),
            include_dir.join("fuse_log.h"),
            include_dir.join("fuse_lowlevel.h"),
            include_dir.join("fuse_opt.h"),
        ];

        Artifacts {
            lib_dir: build_dir.join("lib"),
            includes,
        }
    }

    fn run_command(&self, mut command: Command, desc: &str) {
        println!("running {:?}", command);
        let status = command.status();

        let (status_or_failed, error) = match status {
            Ok(status) if status.success() => return,
            Ok(status) => ("Exit status", format!("{}", status)),
            Err(failed) => ("Failed to execute", format!("{}", failed)),
        };
        panic!(
            "


Error {}:
    Command: {:?}
    {}: {}


    ",
            desc, command, status_or_failed, error
        );
    }
}

fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("libfuse")
}
