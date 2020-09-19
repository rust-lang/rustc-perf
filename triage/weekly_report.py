#!/usr/bin/env python3

from datetime import date
from math import log
from pprint import pp
from sys import exit
import argparse
import json
import msgpack
import sys
import urllib.request

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
    'regressed': [],
    'improved': [],
    'mixed': [],
}


def relative_change(a, b):
    '''Returns `ln(a / b)`

    This is prefereable to percentage change, since order doesn't matter and it
    scales equally for positive or negative changes.

    For small changes, `ln(a / b) ≈ (b - a) / b`
    '''

    return log(a / b)


def gh_link(pr):
    return f'https://github.com/rust-lang/rust/issues/{pr}'


def compare_link(start, end, stat='instructions:u'):
    return f'https://perf.rust-lang.org/compare.html?start={start}&end={end}&stat={stat}'


def change_summary(change, status):
    pr = change['b']['pr']
    direction = status.capitalize()
    stat = 'instruction counts'
    start = change['a']['commit']
    end = change['b']['commit']

    return '\n'.join([
        f'[#{pr}]({gh_link(pr)})'
        f'- {direction} results in [{stat}]({compare_link(start, end)}).'])


def make_request_payload(start, end):
    payload = {
        'start': start,
        'end': end,
        'stat': 'instructions:u',
    }
    return bytes(json.dumps(payload), 'ascii')


def make_request(start, end):
    req = urllib.request.Request('https://perf.rust-lang.org/perf/get')
    req.add_header('Content-Type', 'application/json')
    req.data = make_request_payload(start, end)
    with urllib.request.urlopen(req) as f:
        data = msgpack.unpack(f, raw=False)
        return data


def do_triage(start):
    # Get the next commit after `start` by comparing it with itself
    initial_response = make_request(start, start)

    if initial_response['next'] is None:
        print('Failed to get first commit', file=sys.stderr)
        sys.exit(1)

    commits = [start, initial_response['next']]

    while True:
        try:
            response = make_request(*commits)
        except urllib.error.HTTPError as e:
            print(e, file=sys.stderr)
            break

        if not response['is_contiguous']:
            print('Reached a commit whose perf run is not yet complete',
                  file=sys.stderr)
            break

        handle_compare(response)

        if 'next' not in response:
            break

        commits[0], commits[1] = commits[1], response['next']

    print(report.format(
        first_commit=start, last_commit=commits[0],
        date=date.today().strftime("%Y-%m-%d"),
        num_regressions=len(results['regressed']),
        num_improvements=len(results['improved']),
        num_mixed=len(results['mixed']),
        num_rollups='???',
        regressions='\n\n'.join(results['regressed']),
        improvements='\n\n'.join(results['improved']),
        mixed='\n\n'.join(results['mixed']),
        username='ecstaticmorse'
    ))


def handle_compare(res):
    print(f"Comparing {res['a']['commit']}..{res['b']['commit']}", file=sys.stderr)
    CHANGE_THRESHOLD = 0.01

    data = [res[key]['data'] for key in ['a', 'b']]

    max_regression = 0
    max_improvement = 0
    for bench_name in data[0].keys() & data[1].keys():
        # Ignore rustdoc benchmarks for now
        if bench_name.endswith('-doc'):
            continue

        benches = [dict(datum[bench_name]) for datum in data]
        for cache_state in benches[0].keys() & benches[0].keys():
            measurements = [bench[cache_state] for bench in benches]
            rel_change = relative_change(*measurements)
            max_regression = min(max_regression, rel_change)
            max_improvement = max(max_improvement, rel_change)

    improved = abs(max_improvement) > CHANGE_THRESHOLD
    regressed = abs(max_regression) > CHANGE_THRESHOLD

    if improved and regressed:
        status = 'mixed'
    elif improved:
        status = 'improved'
    elif regressed:
        status = 'regressed'
    else:
        return

    handle_significant_change(res, status)


def handle_significant_change(res, status):
    results[status].append(change_summary(res, status))


if __name__ == '__main__' and not sys.flags.inspect:
    parser = argparse.ArgumentParser(description='Generate a weekly triage report')
    parser.add_argument('first_commit', help="the last commit of last week's triage report")
    args = parser.parse_args()

    do_triage(start=args.first_commit)
