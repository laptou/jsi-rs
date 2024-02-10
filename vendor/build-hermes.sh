#!/bin/bash
set -euxo pipefail
cd hermes
cmake -S . -B build -G Ninja
cmake --build ./build
