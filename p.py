#!/usr/bin/env python
#
# An utility script profiling individual benchmark using CI artifacts:
#
# * Downloads and install CI artifacts using rustup-toolchain-install-master
#   (requires `cargo install rustup-toolchain-install-master`).
# * Runs collector profile_local with selected benchmark and profiler.
# * When using cachegrind automatically compares results using cg_diff.
# * Stores profiler output in results directory.
#
# For example to compare two builds on ctfe-stress-4-doc benchmark using
# cachegrind:
#
# ./p.py -p cachegrind -b ctfe-stress-4 --builds Doc \
#    7611fe438dae91084d17022e705bf64374d5ba4b \
#    bcfd3f7e88084850f87b8e34b4dcb9fceb872d00

import argparse
import os
import pathlib
import shlex
import subprocess
import sys


def check_call(args, *, verbose):
    if verbose:
        print(shlex.join(args))
    return subprocess.check_call(args)


def check_output(args, *, verbose, **kwargs):
    if verbose:
        print(shlex.join(args))
    return subprocess.check_output(args, **kwargs)


def get_rustup_toolchains(*, verbose):
    args = ['rustup', 'toolchain', 'list']
    output = check_output(args, text=True, verbose=verbose)
    toolchains = [line.split()[0] for line in output.splitlines()]
    return toolchains


def get_rustc_path(toolchain, *, verbose):
    args = ['rustup', 'which', '--toolchain', toolchain, 'rustc']
    output = check_output(args, text=True, verbose=verbose)
    return output.rstrip()


def install_toolchains(toolchains, *, verbose):
    """Installs missing toolchains using rustup-toolchain-install-master"""
    installed = get_rustup_toolchains(verbose=verbose)
    for toolchain in toolchains:
        if toolchain in installed:
            continue
        args = ['rustup-toolchain-install-master', toolchain]
        check_call(args, verbose=verbose)


def cachegrind_diff(options):
    if len(options.toolchain) != 2:
        return
    [a, b] = options.toolchain

    benchmark = options.benchmark
    results = pathlib.Path('results')

    for build in options.builds:
        for run in options.runs:
            result_a = results / f'cgout-{a}-{benchmark}-{build}-{run}'
            result_b = results / f'cgout-{b}-{benchmark}-{build}-{run}'

            args = ['cg_diff',
                    '--mod-filename=s#/rustc/[^/]*/##',
                    '--mod-funcname=s/[.]llvm[.].*//',
                    str(result_a),
                    str(result_b)]
            diff = check_output(args, verbose=options.verbose)
            diff_path = results / f'cgdiff-{a}-{b}-{benchmark}-{build}-{run}'
            with open(diff_path, 'wb') as fh:
                fh.write(diff)

            args = ['cg_annotate', str(diff_path)]
            annotated = check_output(args, verbose=options.verbose)
            annotated_path = results / f'cgann-cgdiff-{a}-{b}-{benchmark}-{build}-{run}'
            with open(annotated_path, 'wb') as fh:
                fh.write(annotated)


def main():
    directory = pathlib.Path(__file__).parent
    collector = directory / 'target' / 'release' / 'collector'
    if not collector.exists():
        print('{} not found. Use `cargo build --release` to build it.'.format(collector), file=sys.stderr)
        sys.exit(1)
    collector = str(collector.relative_to(directory))
    os.chdir(directory)

    parser = argparse.ArgumentParser()
    parser.add_argument('-v', '--verbose', action='store_true', default=False)
    parser.add_argument('-b', '--benchmark', required=True, help='benchmark name: ctfe-stress-4, inflate, etc.')
    parser.add_argument('-p', '--profiler', action='append')
    parser.add_argument('--builds', choices=['Check', 'Debug', 'Opt', 'Doc'], action='append')
    parser.add_argument('--runs', choices=['Full', 'IncrFull', 'IncrUnchanged', 'IncrPatched', 'All'], action='append')
    parser.add_argument('toolchain', nargs='+', default=[], help='rustup toolchain name or a commit hash')

    options = parser.parse_args()
    options.profiler = options.profiler or ['cachegrind', 'perf-record']
    options.builds = options.builds or ['Check']
    options.runs = options.runs or ['Full']

    install_toolchains(options.toolchain, verbose=options.verbose)

    for profiler in options.profiler:
        for toolchain in options.toolchain:
            rustc_path = get_rustc_path(toolchain, verbose=options.verbose)
            args = [collector, 'profile_local', profiler, rustc_path]
            args.append(toolchain)
            args.append('--builds')
            args.append(','.join(options.builds))
            args.append('--include')
            args.append(options.benchmark)
            args.append('--runs')
            args.append(','.join(options.runs))
            check_call(args, verbose=options.verbose)
        if profiler == 'cachegrind':
            cachegrind_diff(options)

if __name__ == '__main__':
    main()
