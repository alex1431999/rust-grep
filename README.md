# Rust Grep

This is a very small project to replicate the `grep` command in rust.

I built this project as a learning exercise to improve my rust skills.

## Functionality

You can give the tool a string and a starting directory. It will go through
all the readable files in that directory + subdirectories to find lines
that include the string.

It will then show you all the files + lines where it found the string
and highlight said string in those lines.

## Learning outcomes

The main goal of this project was to get more familiar with the file system
using rust.

I was actually pleasantly surprised by how easy it is to generate temporary files
in the tests to test the code. I was expecting the testing to be annoying since we would
need to simulate a file system, but turns out that's super easy in rust :)

Iterating over files themselves via streams was also pretty straight forward.

In one of the tests I also added a file with 100k lines and the execution was still instant
which was great to see. This is where you can really tell the differences to Typescript (which has been my main
language).

## Usage of AI

All of this has been coded by hand and nothing is AI generated, since this
is a learning project, not an actual production tool.