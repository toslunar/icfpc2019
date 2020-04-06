#!/bin/bash
set -ex

BLOCK=$1
MINING_ROOT="/Users/tos/Dropbox/ICFPC2019/mining"
PUZZLE_IN_PATH="${MINING_ROOT}/in/puzzle_${BLOCK}.cond"
PUZZLE_OUT_PATH="${MINING_ROOT}/out/puzzle_${BLOCK}.desc"

cp ${PUZZLE_IN_PATH} new.cond
cargo run new.cond new.desc

# cargo run "${PUZZLE_IN_PATH}" "${PUZZLE_OUT_PATH}"
