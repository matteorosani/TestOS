#!/bin/bash
CWD=$(pwd)
cargo bootimage
[ ! -e "$CWD/target/x86_64-test_os/debug/bootimage-test_os-pad.bin" ] || rm "$CWD/target/x86_64-test_os/debug/bootimage-test_os-pad.bin"
[ ! -e "$CWD/target/x86_64-test_os/debug/bootimage-test_os.vdi" ] || rm "$CWD/target/x86_64-test_os/debug/bootimage-test_os.vdi"
dd if="$CWD/target/x86_64-test_os/debug/bootimage-test_os.bin" of="$CWD/target/x86_64-test_os/debug/bootimage-test_os-pad.bin" bs=100M conv=sync
VBoxManage convertdd "$CWD/target/x86_64-test_os/debug/bootimage-test_os-pad.bin" "$CWD/target/x86_64-test_os/debug/bootimage-test_os.vdi" --format VDI