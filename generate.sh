#!/usr/bin/env bash

set -ex

pushd mutter-gir-files
rm -fv *.gir
cp -v /usr/share/gir-1.0/GDesktopEnums-3.0.gir . && patch -p1 < GDesktopEnums-3.0.patch
cp -v /usr/share/gir-1.0/Json-1.0.gir .
cp -v /usr/share/gir-1.0/xlib-2.0.gir . && patch -p1 < xlib-2.0.patch
cp -v /usr/lib/x86_64-linux-gnu/mutter-8/Cogl-8.gir . && patch -p1 < Cogl-8.patch
cp -v /usr/lib/x86_64-linux-gnu/mutter-8/CoglPango-8.gir .
cp -v /usr/lib/x86_64-linux-gnu/mutter-8/Clutter-8.gir . && patch -p1 < Clutter-8.patch
cp -v /usr/lib/x86_64-linux-gnu/mutter-8/Meta-8.gir . && patch -p1 < Meta-8.patch
popd

# Packages to generate, listed in dependency order
sys_pkgs=(
    xlib-sys
    cogl-sys
    json-sys
    clutter-sys
    gdesktop_enums-sys
    meta-sys
)

for pkg in "${sys_pkgs[@]}"
do
    # Uncomment to rebuild all automatically generated files
    #rm -rfv "${pkg}"
    if [ ! -d "${pkg}" ]
    then
        cargo run --release --manifest-path gir/Cargo.toml -- \
            --config "${pkg}.toml" \
            --girs-directories mutter-gir-files \
            --girs-directories gir-files
        if [ -f "${pkg}.patch" ]
        then
            pushd "${pkg}"
            patch -p1 < "../${pkg}.patch"
            popd
        fi
    fi
    cargo build --release --manifest-path "${pkg}/Cargo.toml" --all-features
done
