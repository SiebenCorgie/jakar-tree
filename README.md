# jakar-tree
A rust based graph system which can handle many nodes quiet fast.

## How does it work?

### Storing and reading
The system is based on the idea of an "registry" system. You got a `tree` which can hold several `node`s in a hierachy.
The tree struct has a `BTreeMap` which holds the path to a node, keyed by its name. This way, if you want to retrieve a node,
the system can go the "perfect" way to the node.

### Jobs
You can assign jobs to a node. A job (like "move 50 units on the x axis") is executed when the `update()` function is called on
this node or tree. A job get distributed to the children, which means, if you move the parent 50 units, all children will move the 50
units as well and **After** that execute their own jobs.

## How fast is it?
Well, I tested the system with ~50 nodes at two levels. The average getting process per node was around 500 nano seconds (on a
Ryzen 1700x @ 3.4 GHz), which is okay I guess, at least for a single threaded system.

## Where should it be used?
I am currently using it in a game engine. However, the base structs and traits (`node` and `tree`) can be used everywhere.
Most of the internal stuff uses generics, so it should be no problem to implement the system for use somewhere else as well.

## How do I test?
The easiest way is to clone the repository and run in either debug or release mode:
```
$ git clone https://github.com/SiebenCorgie/jakar-tree.git
$ cd jakar-tree/
```
for release mode do:
```
$ cargo run --example simple --release
```
for debug mode do:
```
cargo run --example simple
```

**There is also a speed test with a bit bigger tree and the use of tick function**
```
cargo run --example speed
```
