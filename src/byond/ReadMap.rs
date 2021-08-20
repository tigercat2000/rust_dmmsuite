use super::*;
use crate::{Coord, PrefabList, DMM};

pub(crate) fn parse_and_load(offset: Coord, file_contents: &str) -> Result<(), Runtime> {
    let map = DMM::read_map(file_contents);

    let max_coord = map.calculate_bounds(offset);

    enforce_bounds(max_coord)?;

    let new_atom = Proc::find(byond_string!("/proc/__dmmsuite_new_atom")).unwrap();

    map.map.iter().for_each(|(c, p)| {
        // Safety: We know these pointers are valid because we own the thing they point to.
        let prefab: &PrefabList = unsafe { &**p };

        let c = *c + (offset - Coord(1, 1, 1));

        prefab.prefabs.iter().for_each(|path| {
            new_atom.call(&[
                &Value::from_string(path).unwrap(),
                &Value::from(c.0),
                &Value::from(c.1),
                &Value::from(c.2),
            ]);
        });
    });

    Ok(())
}

fn enforce_bounds(max_coord: Coord) -> Result<(), Runtime> {
    let world = Value::world();

    let maxx = world.get_number(byond_string!("maxx"))? as u32;
    let maxy = world.get_number(byond_string!("maxy"))? as u32;
    let maxz = world.get_number(byond_string!("maxz"))? as u32;

    if maxx < max_coord.0 {
        Value::world().set(byond_string!("maxx"), max_coord.0);
    }
    if maxy < max_coord.1 {
        Value::world().set(byond_string!("maxy"), max_coord.1);
    }
    if maxz < max_coord.2 {
        Value::world().set(byond_string!("maxz"), max_coord.2);
    }

    Ok(())
}
