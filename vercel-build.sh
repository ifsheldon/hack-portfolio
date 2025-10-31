#!/bin/bash

# This is the build script for vercel deployment

export PATH="$HOME/.cargo/bin:$PATH"
./dx bundle --web --ssg --release

# remember to change "hack-portfolio" to your project name
mv target/dx/hack-portfolio/release/web/public .
