#!/bin/bash

commit_hash="c74251226a8caa0b43377902ee06d2570faa0c15"
git clone https://github.com/google/perfetto.git perfetto_tmp

cd perfetto_tmp
git checkout $commit_hash || exit 1

function allow_url_in_csp() {
    url=$1
    sed -i.bak "1,/^[[:space:]]*'blob:'/s/^\([[:space:]]*\)'blob:'/\1'$(echo "$url" | sed 's/\//\\\//g')',\n\1'blob:'/" ui/src/frontend/index.ts
}

allow_url_in_csp 'https://perf.rust-lang.org'
allow_url_in_csp 'http://localhost:2346'

patch -u ui/src/frontend/trace_url_handler.ts < ../perfetto_trace_url_handler.patch

tools/install-build-deps --ui
ui/build

cd ..

rm -rf static/perfetto
mkdir -p static/perfetto
mv perfetto_tmp/out/ui/ui/dist/* static/perfetto/

if [ "$DEBUG_EMBEDDED_PERFETTO" != true ] ; then
    echo "Removing temporal perfetto repository"
    rm -rf perfetto_tmp
fi