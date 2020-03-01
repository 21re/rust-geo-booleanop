#!/bin/bash

# Generates a PDF containing all test cases that have changed w.r.t. master.

cd $(dirname $0)/../..

echo "Generating test cases in $(pwd)"

./tests/scripts/plot_test_cases.py $(git diff --name-only master -- tests/fixtures/generic_test_cases/)
