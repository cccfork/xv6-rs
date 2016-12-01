#!/usr/bin/env bash

bindgen --use-core --output=$1.rs $1 -- -m32
