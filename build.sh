#!/bin/sh

RUSTFLAGS="-C target-cpu=native" cargo install --path .
