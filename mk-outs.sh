#!/usr/bin/env bash

ROOT="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $ROOT/*.txt; do
    BASENAME=$(basename "$FILE")
    uniq      $FILE > ${OUT_DIR}/${BASENAME}.out
    uniq -c   $FILE > ${OUT_DIR}/${BASENAME}.c.out
done
