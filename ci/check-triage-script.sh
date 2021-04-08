#!/bin/bash

set -ex;

pip3 install --user msgpack

report=$(mktemp)
first_commit=4896450e7e0a522486b4d3a8d360ac4e1d2072a0
last_commit=d32238532138485c80db4f2cd596372bce214e00

triage/weekly_report.py >"$report" $first_commit $last_commit

function grep_report {
    grep "$report" >/dev/null -e "$1"
}

grep_report '^2 Regressions, 7 Improvements, 3 Mixed'
grep_report 'regression .*end=36bcf4069717b9dff90270d13b53a3b130329960'
grep_report 'regression .*end=2616ab1c57e2d69f989307389b27ee996ed82575'
grep_report 'improvement .*end=803ddb83598838fb9de308d283b759ba463e5e80'
grep_report 'improvement .*end=0978a9eb99eca9f99889014b232f67ece898aadb'
grep_report 'improvement .*end=138fd56cf9598b4bf016634c768dca128a83a5d7'
grep_report 'improvement .*end=0b417ab5cdfdedffd74fb22cf22d27033c851304'
grep_report 'improvement .*end=35aa636159b84a771000ee025828798fd959933d'
grep_report 'improvement .*end=d203fceeb12f8c0e3123fc45036575018d2f990a'
grep_report 'improvement .*end=d32238532138485c80db4f2cd596372bce214e00'
grep_report 'regression .*end=d474075a8f28ae9a410e95d849d009006db4b176'
grep_report 'improvement .*end=d474075a8f28ae9a410e95d849d009006db4b176'
grep_report 'regression .*end=d1065e6cefa41fe6c55c9819552cdd61529096fc'
grep_report 'improvement .*end=d1065e6cefa41fe6c55c9819552cdd61529096fc'
grep_report 'regression .*end=015d2bc3fec48cef3cbfaec71c54fa31ce420853'
grep_report 'improvement .*end=015d2bc3fec48cef3cbfaec71c54fa31ce420853'

rm -f "$report"
