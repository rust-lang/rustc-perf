#!/bin/bash

set -ex;

pip3 install --user msgpack

report=$(mktemp)
first_commit=4e8a8b49ae57233bc196f3529f5184bc208c3034
last_commit=f68e08933d8f519a9655934fedebbc509661b219

triage/weekly_report.py >"$report" $first_commit $last_commit

function grep_report {
    grep "$report" >/dev/null -e "$1"
}

grep_report '^1 Regressions, 2 Improvements'
grep_report 'regression .*end=8e9d5db8392c44a2e94008168fa3506ecddaa357'
grep_report 'improvement .*end=b3aae050cd7e0c9a9eb6085bd49b02f67dc1396f'
grep_report 'improvement .*end=f68e08933d8f519a9655934fedebbc509661b219'

rm -f "$report"
