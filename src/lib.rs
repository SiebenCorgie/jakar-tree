//! # Jakar-Tree
//!
//! This library can be used to implement any kind of tree graph with a job system.
//! The user can, and should implement his own:
//!
//! - Value type / Content type
//!
//! - Attribute type
//!
//! - Job type
//!
//! - (if you want to compare nodes to a Comparer) a Comparer type (usally the Attribute struct but with `Option<T>` instead of `T` )
//!
//! You can find a sample implementation of each type in the `game_tree` module.
//! There is also a working example in the example directory which shows how to efficently store
//! 100 nodes in 2 layer and call them as well as how to apply jobs to them.


///This module describes the primary tree.
pub mod tree;
///This module describes
pub mod node;
///Contains a example implementation for an 3D scene tree of objects
pub mod game_tree;
