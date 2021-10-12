#!/bin/sh

# This is a helper script that:
# 1. tries to infer the name of the previous week's triage report
# 2. extracts the PARENT commmit from the inferred triage report
# 3. runs curl against the perf site with the extracted PARENT, piping the output to a file for today
# 4. does a git checkout of a branch for today's triage

# Infer previous week's triage report: look for files from 6, 7, and 8 days ago.
# (Usually it will be 7.)

LAST_WEEK_FILE=$(for i in 6 7 8 ; do ls $(printf "%04d-%02d-%02d.md " $(date +%Y) $(date +%m) $(echo $(date +%d) - $i | bc)) 2> /dev/null ; done)
echo "last week: $LAST_WEEK_FILE"

# Turn on exit error code checking. (We cannot do it above because the `ls` invocation above includes ones with error exits.)
set -e


# Extract the PARENT commit from last week's report.
PARENT=$(grep 'Revision range' $LAST_WEEK_FILE | sed -e 's/^.*\[[0-9a-f]*\.\.\([0-9a-f]*\)\].*$/\1/')
echo "parent: $PARENT"

CURL_CMD='curl "https://perf.rust-lang.org/perf/triage?start=$PARENT"'
TODAY_MD=$(printf "%04d-%02d-%02d.md" $(date +%Y) $(date +%m) $(date +%d))

# Ask user if they want to actually do the `curl` command based on the inferred and extracted values.
read -p "Run \`$CURL_CMD > $TODAY_MD\` (y/N)? " YN

case $YN in
    [Nn]* | "" )
        echo "Not running \`curl\`"
        exit 0
        ;;
    [Yy]* )
        echo "Running \`curl\` with output to $TODAY_MD"
        ;;
    * )
        echo "Must answer yes or no"
        exit 1
        ;;
esac

# Avoid overwriting a pre-existing file (we don't want to clober an in-progress report for today!)
if [ -f $TODAY_MD ] ; then
    echo "Not overwriting existing file $TODAY_MD"
    exit 1
fi

# Finally: Generate the report from perf site!
curl "https://perf.rust-lang.org/perf/triage?start=$PARENT" > $TODAY_MD

TODAY_BRANCH=$(printf "triage-%04d-%02d-%02d" $(date +%Y) $(date +%m) $(date +%d))

# Ask user if they want to also make a branch to hold the report.
read -p "Checkout $TODAY_BRANCH branch for doing triage today (y/N)? " YN

case $YN in
    [Nn]* | "" )
        echo "Not running \`git\`"
        exit 0
        ;;
    [Yy]* )
        echo "Running \`git checkout -b $TODAY_BRANCH\`"
        git checkout -b $TODAY_BRANCH
        ;;
    * )
        echo "Must answer yes or no"
        exit 1
        ;;
esac
