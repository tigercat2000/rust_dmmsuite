/proc/pass()
	return

#define WARNING(MSG) warning("[MSG] in [__FILE__] at line [__LINE__] src: [src] usr: [usr].")
/proc/warning(msg)
	world.log << "## WARNING: [msg]"

// These need to exist.
/proc/_process_callbacks()
/proc/_process_callbacks_priority()

/proc/auxtools_stack_trace(msg)
	CRASH(msg)