#include "deps/dmbot.dm"
#include "deps/etc.dm"
#include "deps/globals.dm"
#include "deps/controllers/globals.dm"

#include "../../target/dmmsuite.dm"

/world/New()
	if(!GLOB)
		new /datum/controller/global_vars

	DMMSUITE_CHECK

	for(var/func in typesof(/test/proc))
		log << "[func] [copytext("------------------------------------------------------------------------", length("[func]"))]"
		call(new /test, func)()

		world.maxx = 0
		world.maxy = 0
		world.maxz = 0

	DMMSUITE_SHUTDOWN

	if(world.system_type == MS_WINDOWS)
		shell("taskkill /f /im dreamdaemon.exe")
	else
		del(src)