#!/bin/sh

git remote add -f $1 ../staging/nature-of-code/0.Introduction/$1/.git
git merge --allow-unrelated-histories $1/master
mkdir -p 0.Introduction/$1
git mv .gitignore Cargo.* rustfmt.toml src 0.Introduction/$1
git add .
git commit -m "Moved '$1' into place."
git remote remove $1

