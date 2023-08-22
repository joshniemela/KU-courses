#!/bin/bash

cargo clippy -- \
    -W clippy::pedantic \
    -W clippy::nursery \
    -W clippy::unwrap_used \
    -W clippy::expect_used
