GLOBAL_REAL(GLOB, /datum/controller/global_vars)

/datum/controller/global_vars
	var/list/gvars_datum_protected_varlist
	var/list/gvars_datum_in_built_vars
	var/list/gvars_datum_init_order

/datum/controller/global_vars/New()
	if(GLOB)
		CRASH("Multiple instances of global variable controller created")
	GLOB = src

	var/datum/controller/exclude_these = new
	gvars_datum_in_built_vars = exclude_these.vars + list("gvars_datum_protected_varlist", "gvars_datum_in_built_vars", "gvars_datum_init_order")

	world.log << "[vars.len - gvars_datum_in_built_vars.len] global variables"

	Initialize(exclude_these)

/datum/controller/global_vars/proc/Initialize(var/exclude_these)
	gvars_datum_init_order = list()
	gvars_datum_protected_varlist = list("gvars_datum_protected_varlist")

	for(var/I in (vars - gvars_datum_in_built_vars))
		var/start_tick = world.time
		call(src, "InitGlobal[I]")()
		var/end_tick = world.time
		if(end_tick - start_tick)
			warning("Global [I] slept during initialization!")