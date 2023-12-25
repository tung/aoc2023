#!/bin/sh
#
# Create an input.dot file from input.txt.
#
# First line: digraph day25 {
# Last line: }
#
# Convert lines like so:
#
# abc: def ghi jkl
# abc -> def, ghi, jkl
#
# Look for the three edges that split the groups.
# Copy input.dot to input2.dot and remove those edges,
# then run analyze.sh to work out the answer.
#
neato -Tsvg input.dot > day25.svg
