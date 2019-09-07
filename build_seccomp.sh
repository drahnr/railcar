#!/bin/bash

set -e

function die {
    echo $1
    exit 1
}

[ -e libseccomp/autogen.sh ] || die "libseccomp empty, did you git submodule update --init"
cd libseccomp
[ -e ./configure ] || ./autogen.sh || die "autogen failed, check dependencies"
[ -e ./Makefile ] || CC="musl-gcc" CFLAGS="-fPIC" CPPFLAGS="${CFLAGS} -idirafter/usr/include/x86_64-linux-gnu -idirafter/usr/include" ./configure --enable-static --disable-shared

make || die "make failed, check dependencies"
echo "$PWD/src/.libs"
