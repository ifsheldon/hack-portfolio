#!/bin/bash

# This is the build script for vercel deployment

export PATH="$HOME/.cargo/bin:$PATH"
./dx bundle --platform web --ssg

# remember to change "hack-portfolio" to your project name
mv target/dx/hack-portfolio/release/web/public .
