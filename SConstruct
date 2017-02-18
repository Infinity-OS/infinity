# Release information
version = {
	'PULSAR_VER_RELEASE':  0,
	'PULSAR_VER_UPDATE':   0,
	'PULSAR_VER_REVISION': 0
}

# GCC-specific build flags.
gcc_flags = {
    'CCFLAGS': ['-Wno-unused-but-set-variable'],
}

# Clang-specific build flags.
clang_flags = {
    # Clang's integrated assembler doesn't support 16-bit code.
    'ASFLAGS': ['-no-integrated-as'],
}

# C/C++ warning flags.
cc_warning_flags = [
	'-Wall', '-Wextra', '-Wno-variadic-macros', '-Wno-unused-parameter',
	'-Wwrite-strings', '-Wmissing-declarations', '-Wredundant-decls',
	'-Wno-format', '-Werror', '-Wno-error=unused',
]

# C++ warning flags.
cxx_warning_flags = [
	'-Wold-style-cast', '-Wsign-promo',
]

# Build flags for both host and target.
build_flags = {
    'CCFLAGS': [
        '-Wall', '-Wextra', '-Wno-variadic-macros', '-Wno-unused-parameter',
        '-Wwrite-strings', '-Wmissing-declarations', '-Wredundant-decls',
        '-Wno-format', '-Werror', '-Wno-error=unused', '-pipe',
    ],
    'CFLAGS': ['-std=gnu99'],
    'ASFLAGS': ['-D__ASM__'],
}

# Variables to set in host environments. Don't build C code with our normal
# warning flags, Kconfig and Flex/Bison code won't compile with them. Also
# older host G++ versions don't support some flags.
host_flags = {
	'CCFLAGS': ['-pipe'],
	'CFLAGS': ['-std=gnu99'],
	'CXXFLAGS': filter(lambda f: f not in [
		'-Wmissing-declarations', '-Wno-variadic-macros',
		'-Wno-unused-but-set-variable'], cc_warning_flags),
	'YACCFLAGS': ['-d'],
}

# Build flags for target.
target_flags = {
    'CCFLAGS': [
        '-gdwarf-2', '-pipe', '-nostdlib', '-nostdinc', '-ffreestanding',
        '-fno-stack-protector', '-Os', '-fno-omit-frame-pointer',
        '-fno-optimize-sibling-calls'
    ],
    'ASFLAGS': ['-nostdinc'],
    'LINKFLAGS': [],
}

################################################################################
# Internal build setup.                                                        #
################################################################################

import os, sys

# Add the path to our build utilities to the PATH
sys.path = [os.path.abspath(os.path.join('utilities', 'build'))] + sys.path

from kconfig import ConfigParser
from manager import BuildManager
from toolchain import ToolchainManager
import vcs
from util import RequireTarget
from subprocess import Popen, PIPE

# Set the version string.
version['PULSAR_VER_STRING'] = '%d.%d' % (
	version['PULSAR_VER_RELEASE'],
	version['PULSAR_VER_UPDATE'])
if version['PULSAR_VER_REVISION']:
	version['PULSAR_VER_STRING'] += '.%d' % (version['PULSAR_VER_REVISION'])
revision = vcs.revision_id()
if revision:
	version['PULSAR_VER_STRING'] += '-%s' % (revision)

# Change the Decider to MD5-timestamp to speed up the build a bit.
Decider('MD5-timestamp')

# Create the build environment
host_env = Environment(ENV = os.environ)
target_env = Environment(platform = 'posix', ENV = os.environ)

# Merge the build flags on the both environments
for (k, v) in build_flags.items():
    host_env[k] = v
    target_env[k] = v

# Create the build manager
manager = BuildManager(host_env, target_env)

# Load the build configuration (if it exists)
config = ConfigParser('.config')
manager.AddVariable('_CONFIG', config)

# Make config, and the manager available outside this script
Export('config', 'manager')

# Add compiler-specific flags.
output = Popen(["clang", "--version"], stdout=PIPE).communicate()[0]
host_env['IS_CLANG'] = output.find('clang') >= 0
if host_env['IS_CLANG']:
    for (k, v) in clang_flags.items():
        host_env[k] += v
else:
    for (k, v) in gcc_flags.items():
        host_env[k] += v

# Set up the host environment template
for (k, v) in host_flags.items():
	host_env[k] = v

# darwin hosts probably have needed libraries in /opt
if os.uname()[0] == 'Darwin':
	host_env['CPPPATH'] = ['/opt/local/include']
	host_env['LIBPATH'] = ['/opt/local/lib']

# Cretae yhe host environment and build host utilities.
env = manager.CreateHost(name = 'host')
SConscript('utilities/SConscript', variant_dir = os.path.join('build', 'host'), exports = ['env'])

# Add targets to run the configuration interface
env['ENV']['KERNELVERSION'] = version['PULSAR_VER_STRING']
Alias('config', env.ConfigMenu('__config', ['Kconfig']))

# If the configuration does not exist, all we can do is configure. Raise an
# error to notify the user that they need to configure if they are not trying
# to do so, and don't run the rest of the build.
if not config.configured() or 'config' in COMMAND_LINE_TARGETS:
	RequireTarget('config',
		"Configuration missing or out of date. Please update using 'config' target.")
	Return()

# Initialise the toolchain manager and add the toolchain build target.
toolchain = ToolchainManager(config)
Alias('toolchain', Command('__toolchain', [], Action(toolchain.update, None)))

# If the toolchain is out of date, only allow it to be built.
if toolchain.check() or 'toolchain' in COMMAND_LINE_TARGETS:
	RequireTarget('toolchain',
		"Toolchain out of date. Update using the 'toolchain' target.")
	Return()

################################################################################
# Target                                                                       #
################################################################################

# Override default assembler - it uses as directly, we want to use GCC.
env['ASCOM'] = '$CC $_CCCOMCOM $ASFLAGS -c -o $TARGET $SOURCES'

# Merge in build flags
for (k, v) in target_flags.items():
    target_env[k] += v

# Set paths to toolchain components.
if os.environ.has_key('CC') and os.path.basename(os.environ['CC']) == 'ccc-analyzer':
	target_env['CC'] = os.environ['CC']
	target_env['ENV']['CCC_CC'] = toolchain.tool_path('clang')

	# Force a rebuild when doing static analysis.
	def decide_if_changed(dependency, target, prev_ni):
		return True
	target_env.Decider(decide_if_changed)
else:
	target_env['CC'] = toolchain.tool_path('clang')
if os.environ.has_key('CXX') and os.path.basename(os.environ['CXX']) == 'c++-analyzer':
	target_env['CXX'] = os.environ['CXX']
	target_env['ENV']['CCC_CXX'] = toolchain.tool_path('clang++')
else:
	target_env['CXX'] = toolchain.tool_path('clang++')
target_env['AS']      = toolchain.tool_path('as')
target_env['OBJDUMP'] = toolchain.tool_path('objdump')
target_env['READELF'] = toolchain.tool_path('readelf')
target_env['NM']      = toolchain.tool_path('nm')
target_env['STRIP']   = toolchain.tool_path('strip')
target_env['AR']      = toolchain.tool_path('ar')
target_env['RANLIB']  = toolchain.tool_path('ranlib')
target_env['OBJCOPY'] = toolchain.tool_path('objcopy')
target_env['LD']      = toolchain.tool_path('ld')

# TODO: we must recompile de rust to add support to Pulsar targets
target_env['CARGO']   = 'cargo'

# Build the target system
SConscript('source/SConscript', variant_dir = os.path.join('build',
    '%s-%s' % (config['ARCH'], config['PLATFORM'])))

# Now that we have information of all libraries, update the toolchain sysroot.
toolchain.update_sysroot(manager)
