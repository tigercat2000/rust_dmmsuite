#define DMMSUITE (world.GetConfig("env", "DMMSUITE") || (world.system_type == MS_WINDOWS ? "rust_dmmsuite.dll" : "librust_dmmsuite.so"))

GLOBAL_VAR_INIT(dmmsuite_initialized, FALSE)

#define DMMSUITE_CHECK\
	if(!GLOB.dmmsuite_initialized && fexists(DMMSUITE) && findtext(call(DMMSUITE,"auxtools_init")(),"SUCCESS"))\
		GLOB.dmmsuite_initialized = TRUE;\
		dmmsuite_init();\

#define DMMSUITE_SHUTDOWN\
	if(GLOB.dmmsuite_initialized && fexists(DMMSUITE) && findtext(call(DMMSUITE,"auxtools_shutdown")(),"SUCCESS"))\
		GLOB.dmmsuite_initialized = FALSE;\


/proc/dmmsuite_init()

// Should return "Hello from DMMSuite!"
/proc/dmmsuite_test()
	CRASH("DMMSUITE NOT LOADED")


/proc/dmmsuite_load_map(file, x_offset, y_offset, z_offset)

// /// Shortcut function to parse a map and apply it to the world.
// ///
// /// - `file`: A .dmm file to load (Required).
// /// - `x_offset`, `y_offset`, `z_offset`: Positions representign where to load the map (Optional).
// /// - `x_lower`, `x_upper`, `y_lower`, `y_upper`: Coordinates (relative to the map) to crop to (Optional).
// /proc/dmmsuite_load_map(file, x_offset, y_offset, z_offset, x_lower, x_upper, y_lower, y_upper)

/proc/__dmmsuite_new_atom(textpath, x, y, z)
	var/path = text2path(textpath)
	new path(locate(x,y,z))