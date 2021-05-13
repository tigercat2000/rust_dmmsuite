use super::*;
use std::io::{self, Write};
use std::path::PathBuf;

#[hook("/proc/dmmsuite_init")]
fn dmmsuite_init() {
    Ok(Value::null())
}
#[hook("/proc/dmmsuite_test")]
fn dmmsuite_test() {
    Value::from_string("Hello from DMMSuite!")
}

#[hook("/proc/dmmsuite_load_map")]
fn load_map(x: Value, y: Value, z: Value, file: Value) {
    let x: _ = x.as_number()?;
    let y = y.as_number()?;
    let z = z.as_number()?;
    let file = file.as_string()?;

    // let proc = Proc::find("/proc/auxtools_stack_trace").unwrap();
    // proc.call(&[
    //     &Value::from_string(format!("cwd is {:?}", std::env::current_dir().unwrap())).unwrap(),
    // ]);

    let path = match PathBuf::from(&file).canonicalize() {
        Ok(path) => path,
        Err(p) => return Err(runtime!(format!("Unable to find file {}", file))),
    };

    let file_contents = match std::fs::read_to_string(path) {
        Ok(file) => file,
        Err(p) => return Err(runtime!(format!("Unable to read file {}", file))),
    };

    ReadMap::parse_and_load(x, y, z, &file_contents)?;
    Ok(Value::null())
}
