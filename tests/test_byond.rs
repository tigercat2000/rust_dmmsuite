use std::path::Path;
use std::process::{Command, Output, Stdio};

#[test]
#[ignore]
fn generic_tests() {
    run_dm_tests("generic");
}

/*
#[cfg(feature = "git")]
#[test]
fn git() {
    run_dm_tests("git");
}

#[cfg(feature = "url")]
#[test]
fn url() {
    run_dm_tests("url");
} */

fn run_dm_tests(name: &str) {
    std::env::remove_var("RUST_BACKTRACE");

    let byond_bin_env = std::env::var("BYOND_BIN").expect("environment variable BYOND_BIN");

    let byond_bin = Path::new(&byond_bin_env);
    let byondexec = byond_bin.join("byondexec");
    let dream_maker = if cfg!(windows) {
        byond_bin.join("dm.exe")
    } else {
        byond_bin.join("DreamMaker")
    };
    let dream_daemon = if cfg!(windows) {
        byond_bin.join("dreamdaemon.exe")
    } else {
        byond_bin.join("DreamDaemon")
    };

    let dme = Path::new(".")
        .join("tests")
        .join("dm")
        .join(format!("{}.dme", name));
    let dmb = Path::new(".")
        .join("tests")
        .join("dm")
        .join(format!("{}.dmb", name));
    let rsc = Path::new(".")
        .join("tests")
        .join("dm")
        .join(format!("{}.rsc", name));

    let target_dir = if cfg!(target_os = "linux") {
        "i686-unknown-linux-gnu"
    } else {
        "i686-pc-windows-msvc"
    };

    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    let fname = if cfg!(target_os = "linux") {
        "librust_dmmsuite.so"
    } else {
        "rust_dmmsuite.dll"
    };

    let dmmsuite = Path::new("target")
        .join(target_dir)
        .join(profile)
        .join(fname);

    let output = if cfg!(windows) {
        Command::new(&dream_maker.to_str().unwrap())
            .arg(&dme)
            .output()
            .unwrap()
    } else {
        Command::new("bash")
            .arg(&byondexec)
            .arg(&dream_maker.to_str().unwrap())
            .arg(&dme)
            .output()
            .unwrap()
    };
    dump(&output);
    generic_check(&output);

    let output = if cfg!(windows) {
        Command::new(&dream_daemon)
            .env("DMMSUITE", &dmmsuite.to_str().unwrap())
            .arg(&dmb)
            .arg("-trusted")
            .arg("-cd")
            .arg(env!("CARGO_MANIFEST_DIR"))
            .arg("-logself")
            .output()
            .unwrap()
    } else {
        Command::new("bash")
            .arg(&byondexec)
            .arg(&dream_daemon)
            .arg(&dmb)
            .arg("-trusted")
            .arg("-cd")
            .arg(env!("CARGO_MANIFEST_DIR"))
            .env("DMMSUITE", &dmmsuite.to_str().unwrap())
            .output()
            .unwrap()
    };
    let _ = std::fs::remove_file(&dmb);
    let _ = std::fs::remove_file(&rsc);

    if cfg!(windows) {
        let logpath = Path::new(".").join("tests").join("dm").join("generic.log");
        let log = std::fs::read_to_string(&logpath).expect("Can't read generic.log");
        let _ = std::fs::remove_file(&logpath);

        check_log(log);
    }

    check_output(&output);
}

fn check_log(log: String) {
    eprintln!("-----Log-----\n{}", log);
    runtime_check(&log.into());
}

fn check_output(output: &Output) {
    dump(&output);
    if !cfg!(windows) {
        // We have to nuke the shit out of the dream daemon process to get it to stop
        generic_check(&output);
    }
    runtime_check(&output.stderr);
}

fn dump(output: &Output) {
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

fn generic_check(output: &Output) {
    if !output.status.success() {
        panic!("process exited with {:?}", output.status);
    }
}

fn runtime_check(stderr: &Vec<u8>) {
    for line in stderr.split(|&c| c == b'\n') {
        if line.starts_with(b"runtime error: ") {
            panic!("{}", String::from_utf8_lossy(line));
        }
    }
}
