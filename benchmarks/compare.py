#! /usr/bin/python
#
# Compares the speed of two different compilers on one or more benchmarks in
# the suite.

from __future__ import division, print_function

import os
import subprocess
import sys
import time

def run_test(dir, rustc):
    os.chdir(dir)

    # First clean and rebuild from scratch. This downloads any necessary code
    # and builds preliminary crates.
    #
    # We use call() instead of check_call() for `make clean` because it can
    # fail reasonably, e.g. if we try to delete a Cargo.lock file thta isn't
    # present.
    subprocess.call('make clean > /dev/null 2>&1', shell=True)
    make_env = os.environ.copy()
    make_env['RUSTC'] = rustc
    subprocess.check_call('make > /dev/null 2>&1', env=make_env, shell=True)

    # Measure compilation speed of the crate of interest three times.
    times = []
    for i in range(0, 3):
        subprocess.check_call('make touch > /dev/null 2>&1', shell=True)
        t1 = time.time()
        subprocess.check_call('make > /dev/null 2>&1', env=make_env, shell=True)
        t2 = time.time()
        t = t2 - t1
        times.append(t)

    os.chdir('..')

    return times


if __name__ == '__main__':
    if (len(sys.argv)) < 3:
        print('usage:', sys.argv[0], '<rustc1>', '<rustc2>', '[benchmarks]')
        sys.exit(1)

    rustc1 = sys.argv[1]
    rustc2 = sys.argv[2]

    # Get all the directories, excluding ".git".
    all_dirs = sorted([f for f in os.listdir('.') if os.path.isdir(f) and f != ".git"])

    # Get any specified directories. Otherwise, default to all of them.
    dirs = sys.argv[3:]
    if dirs:
        for dir in dirs:
            if dir not in all_dirs:
                print('error: bad directory specified: \'' + dir + '\'')
                sys.exit(1)
    else:
        dirs = all_dirs

    # Run the requested benchmarks.
    for dir in dirs:
        times1 = run_test(dir, rustc1)
        times2 = run_test(dir, rustc2)

        min1 = min(times1)
        min2 = min(times2)
        max1 = max(times1)
        max2 = max(times2)

        print('{:15s} {:6.3f}s vs {:6.3f}s --> '
              '{:5.3f}x faster (variance: {:5.3f}x, {:5.3f}x)'.
            format(dir[:15], min1, min2, min1 / min2, max1 / min1, max2 / min2))

