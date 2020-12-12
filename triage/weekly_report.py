#!/usr/bin/env python3

from dataclasses import dataclass
from datetime import date
from enum import Enum
from math import log
from os import getenv
from pathlib import Path
from pprint import pp
from sys import exit
from typing import ClassVar, List
import argparse
import json
import sys
import urllib.request

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

try:
    import msgpack
except ImportError as e:
    eprint(e)
    eprint('Try `pip3 install --user msgpack`')
    sys.exit(1)

report = '''
{date} Triage Log

TODO: Summary

Triage done by **@{username}**.
Revision range: [{first_commit}..{last_commit}](https://perf.rust-lang.org/?start={first_commit}&end={last_commit}&absolute=false&stat=instructions%3Au)

{num_regressions} Regressions, {num_improvements} Improvements, {num_mixed} Mixed
{num_rollups} of them in rollups

#### Regressions

{regressions}

#### Improvements

{improvements}

#### Mixed

{mixed}

#### Nags requiring follow up

TODO: Nags

'''

results = {
    'regression': [],
    'improvement': [],
    'mixed': [],
}


def get_username():
    usernames = {'mackendy': 'ecstaticmorse', 'joshua': 'jyn514'}

    home_dir = Path.home().name

    return usernames.get(home_dir) or home_dir


class Metric(Enum):
    INSTRUCTIONS = 'instructions:u'

    def human_readable(self):
        if self == Metric.INSTRUCTIONS:
            return 'instruction count'
        else:
            raise NotImplementedError

    def direction(self, change):
        if self == Metric.INSTRUCTIONS:
            return 'regression' if change > 0 else 'improvement'
        else:
            raise NotImplementedError


def relative_change(expected, actual):
    '''Returns (actual - expected) / expected

    This is the standard definition of relative change.
    '''

    return (actual - expected) / expected


def log_change(expected, actual):
    '''Returns `ln(actual / expected)`

    This is preferable to percentage change because it scales equally for
    positive or negative changes. This means that the order of the arguments
    only affects the sign of the output

    For small changes, `log_change(a, b) ≈ relative_change(a, b)`
    '''

    return log(actual / expected)


@dataclass
class BenchmarkComparison:
    SIGNIFICANCE_THRESHOLD: ClassVar[float] = 0.01

    results: List[float]
    bench_name: str
    cache_state: str

    metric: Metric = Metric.INSTRUCTIONS

    def log_change(self):
        return log_change(*self.results)

    def relative_change(self):
        return relative_change(*self.results)

    def is_significant(self):
        return abs(self.log_change()) > self.__class__.SIGNIFICANCE_THRESHOLD

    def is_increase(self):
        return self.results[1] > self.results[0]

    def direction(self):
        return self.metric.direction(self.log_change())

    def summary_line(self, link):
        magnitude = abs(self.log_change())
        if magnitude > 0.10:
            size = 'Very large'
        elif magnitude > 0.05:
            size = 'Large'
        elif magnitude > 0.01:
            size = 'Moderate'
        elif magnitude > 0.005:
            size = 'Small'
        else:
            size = 'Very small'

        percent = self.relative_change() * 100
        return (
            f'{size} {self.direction()} in [{self.metric.human_readable()}s]({link})'
            f' (up to {percent:.1f}% on `{self.cache_state}` builds of `{self.bench_name}`)'
        )


def get_benchmarks(res):
    ret = []
    data = [res[key]['data'] for key in ['a', 'b']]
    for bench_name in data[0].keys() & data[1].keys():
        # Ignore rustdoc benchmarks for now
        if bench_name.endswith('-doc'):
            continue

        benches = [dict(datum[bench_name]) for datum in data]
        for cache_state in benches[0].keys() & benches[1].keys():
            measurements = [bench[cache_state] for bench in benches]
            comparison = BenchmarkComparison(measurements, bench_name,
                                             cache_state)
            ret.append(comparison)

    return ret


def gh_link(pr):
    return f'https://github.com/rust-lang/rust/issues/{pr}'


def gh_pr_title(pr):
    def make_req():
        url = f'https://api.github.com/repos/rust-lang/rust/pulls/{pr}'
        req = urllib.request.Request(url)
        req.add_header('Content-Type', 'application/json')
        req.add_header('Authorization', f'token {getenv("GITHUB_TOKEN")}')

        with urllib.request.urlopen(req) as f:
            data = json.loads(f.read())
            return data.get('title', '')

    result = ''
    try:
        result = make_req()
    except urllib.error.HTTPError as e:
        eprint(e)
    finally:
        return result


def compare_link(start, end, stat):
    return f'https://perf.rust-lang.org/compare.html?start={start}&end={end}&stat={stat.value}'


def write_section(res, *changes):
    pr = res['b']['pr']
    start = res['a']['commit']
    end = res['b']['commit']
    title = gh_pr_title(pr)

    msg = f'{title}[#{pr}]({gh_link(pr)})'

    for change in changes:
        msg += '\n- '
        msg += change.summary_line(compare_link(start, end, change.metric))

    return msg


def handle_compare(res):
    eprint(f"Comparing {res['b']['commit']} to {res['a']['commit']}")

    benchmarks = get_benchmarks(res)

    # Skip empty commits, sometimes happens if there's a compiler bug or so.
    if len(benchmarks) == 0:
        return

    lo = min(benchmarks, key=lambda x: x.log_change())
    hi = max(benchmarks, key=lambda x: x.log_change())

    changes = []
    if hi.is_increase():
        changes.append(hi)

    if not lo.is_increase():
        changes.append(lo)

    changes = [c for c in changes if c.is_significant()]

    if len(changes) == 0:
        return

    # Unless all changes are going the same direction, report these results as "mixed"
    if len(set(c.is_increase() for c in changes)) == 1:
        section = changes[0].direction()
    else:
        section = 'mixed'

    # Print biggest change first
    changes.sort(reverse=True, key=lambda x: abs(x.log_change()))

    results[section].append(write_section(res, *changes))


def make_request_payload(start, end):
    if start is None:
        start = ""
    if end is None:
        end = ""
    payload = {
        'start': start,
        'end': end,
        'stat': 'instructions:u',
    }
    return bytes(json.dumps(payload), 'ascii')


def make_request(start, end):
    # Retry request twice
    try:
        req = urllib.request.Request('https://perf.rust-lang.org/perf/get')
    except:
        req = urllib.request.Request('https://perf.rust-lang.org/perf/get')

    req.add_header('Content-Type', 'application/json')
    req.data = make_request_payload(start, end)
    with urllib.request.urlopen(req) as f:
        data = msgpack.unpack(f, raw=False)
        return data



def do_triage(start, end):
    # Get the next commit after `start` by comparing it with itself
    initial_response = make_request(start, start)

    if initial_response.get('next') is None:
        eprint('Failed to get first commit')
        sys.exit(1)

    commits = [start, initial_response['next']]

    while True:
        try:
            response = make_request(*commits)
        except urllib.error.HTTPError as e:
            eprint(f"Failed to make request for {commits[0]} and {commits[1]}")
            eprint(e)
            eprint("URL: " + e.geturl())
            eprint("Data: " + e.read().decode())
            break

        if not response['is_contiguous']:
            eprint(f"Reached pair start: {commits[0]} to next: {commits[1]} whose perf run is not yet complete")
            break

        handle_compare(response)
        last_reported = commits[1]

        if 'next' not in response or commits[1] == end:
            break

        commits[0], commits[1] = commits[1], response['next']

    out = report.format(first_commit=start,
                        last_commit=last_reported,
                        date=date.today().strftime("%Y-%m-%d"),
                        num_regressions=len(results['regression']),
                        num_improvements=len(results['improvement']),
                        num_mixed=len(results['mixed']),
                        num_rollups='???',
                        regressions='\n\n'.join(results['regression']),
                        improvements='\n\n'.join(results['improvement']),
                        mixed='\n\n'.join(results['mixed']),
                        username=get_username())

    print(out)


if __name__ == '__main__' and not sys.flags.inspect:
    parser = argparse.ArgumentParser(
        description='Print a weekly triage report to stdout')
    parser.add_argument(
        'first_commit',
        help=
        'The parent of the earliest commit that will be included in the report'
    )
    parser.add_argument(
        'last_commit',
        nargs='?',
        default=None,
        help='The latest commit that will be included in the report')
    args = parser.parse_args()

    do_triage(start=args.first_commit, end=args.last_commit)
