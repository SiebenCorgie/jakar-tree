# jakar-tree
A rust based graph system which can handle many nodes quiet fast.

## How does it work?

### Storing and reading
The system is based on the idea of an "registry" system. You got a `tree` which can hold several `node`s in a hierachy.
The tree struct has a `BTreeMap` which holds the path to a ndoe, keyed by its name. This way, if you want to retrieve a node,
the system can go the "perfect" way to the node.

### Jobs
I plan to a add a "Job" system. The idea is, to be able to add jobs like "Move by 50 units in the x direction" to a node. When the 
`update()` funtion is called, the node applies this operation/job and promotes it down to its children. This way the hierachie get
updated at a defined time and only has to work with each ndoe once.
My old system was updating the system at each call which can become quiet expensive if you got many jobs per frame.

## How fast is it?
Well, I tested the system with ~50 nodes at two levels. The average getting process per node was around 500 nano seconds, 
which is okay I guess, at least for a single threaded system.

## Where should it be used?
I am currently using it in a gema engien. However, the base structs and traits (`node` and `tree`) can be used everywhere.
