#!/bin/sh

set -e

# Convert a PNG to a 1BPP greyscale image
convert $1.png -depth $2 gray:"$1_$2bpp".raw