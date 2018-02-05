#!/bin/sh

set -e

# Convert a PNG to an 8BPP greyscale image
convert samuel.png -depth 8 gray:samuel.raw