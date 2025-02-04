#!/bin/bash

sr="../tg-report"

if [ -d "$sr" ]; then
    cargo build --target x86_64-pc-windows-gnu --release
    nautilus "./target/x86_64-pc-windows-gnu/release"
    cd "$sr" || exit
    cargo build --target x86_64-pc-windows-gnu --release
    nautilus "./target/x86_64-pc-windows-gnu/release"
else
    echo "Второй репозиторий не найден"
fi