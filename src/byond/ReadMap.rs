use super::*;
use crate::DMM;

pub(crate) fn parse_and_load(x: f32, y: f32, z: f32, file_contents: &str) -> Result<(), Runtime> {
    let world = Value::world();

    let maxz = world.get_number(byond_string!("maxz"));
    let maxz = match maxz {
        Err(runtime) => return Err(runtime),
        Ok(v) => v,
    };
    if z > maxz {
        Value::world().set(byond_string!("maxz"), z);
    }

    let map = DMM::read_map(file_contents);

    Ok(())
}
