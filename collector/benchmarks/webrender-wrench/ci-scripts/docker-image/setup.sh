#!/usr/bin/env bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/. */

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

test "$(whoami)" == 'root'

# Install stuff we need
apt-get -y update
apt-get install -y \
    bzip2 \
    cmake \
    curl \
    gcc \
    git \
    g++ \
    libfontconfig1-dev \
    libgl1-mesa-dev \
    libx11-dev \
    openjdk-8-jdk \
    pkg-config \
    python \
    python-mako \
    python-pip \
    python-setuptools \
    python-voluptuous \
    python-yaml \
    software-properties-common

# Get freetype 2.8 with subpixel rendering enabled.
SNAPSHOT_ARCHIVE=http://snapshot.debian.org/archive/debian/20180213T153535Z
FT_SHA=9b4b0c950c211572066561b5e12e5324f1fb34a182ba8239bbd172cede1f090b
FT_DEV_SHA=b76639b55cb8c8bfff1822e54bf5fbeb0bba018e1658a6047f9ac1b9553ddb0b
curl -sSfL -o libfreetype6.deb \
  "${SNAPSHOT_ARCHIVE}/pool/main/f/freetype/libfreetype6_2.8.1-2_amd64.deb"
curl -sSfL -o libfreetype6-dev.deb \
  "${SNAPSHOT_ARCHIVE}/pool/main/f/freetype/libfreetype6-dev_2.8.1-2_amd64.deb"
echo "${FT_SHA}  libfreetype6.deb" | sha256sum --check -
echo "${FT_DEV_SHA}  libfreetype6-dev.deb" | sha256sum --check -
apt install -y ./libfreetype6.deb ./libfreetype6-dev.deb

# Other stuff we need
pip install servo-tidy==0.3.0
