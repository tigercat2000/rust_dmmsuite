use super::*;
use crate::{Coord, DMM};

pub(crate) fn parse_and_load(x: u32, y: u32, z: u32, file_contents: &str) -> Result<(), Runtime> {
    let map = DMM::read_map(file_contents);

    // let (maxx, maxy, maxz) = map.calculate_bounds();

    // enforce_bounds(maxx, maxy, maxz)?;

    let loadable = map.to_loadable(x, y, z);

    let new_atom = Proc::find(byond_string!("/proc/__dmmsuite_new_atom")).unwrap();

    for (coord, item) in loadable {
        new_atom.call(&[
            &Value::from_string(item).unwrap(),
            &Value::from(coord.x),
            &Value::from(coord.y),
            &Value::from(coord.z),
        ]);
    }

    Ok(())
}

fn enforce_bounds(x: u32, y: u32, z: u32) -> Result<(), Runtime> {
    let world = Value::world();

    let maxx = world.get_number(byond_string!("maxx"))? as u32;
    let maxy = world.get_number(byond_string!("maxy"))? as u32;
    let maxz = world.get_number(byond_string!("maxz"))? as u32;

    if maxx < x {
        Value::world().set(byond_string!("maxx"), x);
    }
    if maxy < y {
        Value::world().set(byond_string!("maxy"), y);
    }
    if maxz < z {
        Value::world().set(byond_string!("maxz"), z);
    }

    Ok(())
}
