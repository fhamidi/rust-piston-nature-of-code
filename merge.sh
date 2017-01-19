#!/bin/sh

git remote add -f $1 ../staging/nature-of-code/$1/.git
git merge --allow-unrelated-histories $1/master
mkdir $1
git mv .gitignore Cargo.* rustfmt.toml src $1
git add .
git commit -m "Moved '$1' into place."
git remote remove $1

