#!/usr/bin/env python3

import os, subprocess, sys

def usage_and_exit():
    print("")
    print("Usage:")
    print("    bindings-all.py <platform> <bindgen> <clang_lib_path>")
    print("")
    print("    Regenerates both debugmozjs and non-debugmozjs bindings and")
    print("    tests them.")
    print("")
    print("    <platform>         One of 'linux_32', 'linux_64', 'macos_64',")
    print("                       'freebsd_32', 'freebsd_64',")
    print("                       'windows_gcc_64', or 'windows_msvc14_64'.")
    print("")
    print("    <bindgen>          The path to bindgen")
    print("")
    print("    <clang_lib_path>   The path to the directory of clang library files")
    sys.exit(1)

# Validate arguments.

if len(sys.argv) != 4:
    usage_and_exit()

[platform, bindgen, clang_lib_path] = sys.argv[1:]

try:
    ["linux_32", "linux_64", "freebsd_32", "freebsd_64", "macos_64", "windows_gcc_64", "windows_msvc14_64"].index(platform)
except ValueError:
    print("error: {} is not a valid platform".format(platform))
    usage_and_exit()

bindgen = os.path.abspath(bindgen)
if not (os.path.isfile(bindgen) and os.access(bindgen, os.X_OK)):
    print("error: {} is not executable".format(bindgen))
    usage_and_exit()

clang_lib_path = os.path.abspath(clang_lib_path)
if not os.path.isdir(clang_lib_path):
    print("error: {} is not a directory".format(clang_lib_path))

# Go to the root of our repo.
os.chdir(os.path.dirname(sys.argv[0]))
os.chdir("..")

def run(cmd, **kwargs):
    """Run the given shell command.

    Pass through kwargs (like env=...) to `subprocess.Popen`. Wait for the
    subprocess to complete, and throw an exception if it didn't exit with 0.

    """
    print("{}: Running".format(sys.argv[0]), cmd)
    proc = subprocess.Popen(cmd, **kwargs)
    proc.wait()
    if proc.returncode != 0:
        raise subprocess.CalledProcessError(proc.returncode, cmd)

# Set up the environment needed to run bindgen.
bindgen_env = os.environ.copy()
bindgen_env["LIBCLANG_PATH"] = clang_lib_path
bindgen_env["BINDGEN"] = bindgen
if platform.startswith("macos"):
    bindgen_env["DYLD_LIBRARY_PATH"] = clang_lib_path
else:
    bindgen_env["LD_LIBRARY_PATH"] = clang_lib_path

# Run our builds.

BUILDS = [
    # Release build
    ("", []),
    # DEBUG build
    ("_debug", ["--features", "debugmozjs"])
]

for (build_modifier, extra_cargo_flags) in BUILDS:
    run(["cargo", "clean"])
    run(["cargo", "build", "-p", "mozjs_sys"] + extra_cargo_flags)
    run(["./etc/bindings.sh"], env=bindgen_env)
    run(["mv", "out.rs", "src/jsapi_{}{}.rs".format(platform, build_modifier)])
    run(["cargo", "test"] + extra_cargo_flags)
