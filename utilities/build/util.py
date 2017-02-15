from SCons.Script import *

# Helper for creating source lists with certain files only enabled by config
# settings.
def FeatureSources(config, files):
    output = []
    for f in files:
        if type(f) is tuple:
            if reduce(lambda x, y: x or y, [config[x] for x in f[0:-1]]):
                output.append(File(f[-1]))
        else:
            output.append(File(f))
    return output

# Raise an error if a certain target is not specified.
def RequireTarget(target, error):
	if GetOption('help') or target in COMMAND_LINE_TARGETS:
		return
	raise SCons.Errors.StopError(error)
