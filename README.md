# Rust Set-Cover

## Input and Output

This program is a Clickhouse EUDF (Executable User Defined Function). As such, it takes input from stdin and outputs to stdout.

It expects Clickhouse's [Rowbinary](https://clickhouse.com/docs/en/interfaces/formats/#rowbinary) data in the format `Array(Array(UInt32))` (i.e. `[[1,2,3],[1,2]]`).

It returns boolean output in `UInt8` (either 0 or 1).

## What it Does

Parabible searches involve finding matching words in the same location (syntactical units). Words are matched on a variety of features such that a single word may match more than one set of constraints and be a valid return value for different search terms. For example, Parabible allows the user to search for singular nouns and masculine nouns. Of course, the word "man" satisfies both constraints.

The problem is that for a query to be satisified, there should be some way of assigning a single matching word to each set of constraints. This means that, if the only matching word in a syntactical unit is "man", a search for syntactical units that contain "a singular noun" and "a masculine noun" should not return the unit. This is a type of "set cover" problem (though, not the typical one).

The problem may be understood by way of example:

```
set_cover([1,2],[1,2]) 	     // True
set_cover([1,2],[1])         // True
set_cover([1],[1])           // False
set_cover([1,2],[1,2,3])     // True
set_cover([1,2],[1,2],[3])   // True
set_cover([1,2],[2,3],[3,1]) // True
set_cover([1],[1],[1,2,3])   // False
```

## How it Works

Because we don't need to return samples of possible ways to "cover the set", the problem is simpler to solve.

A simple assertion lies at the heart of the algorithm:

> If there are more arrays passed into the function than the union of their elements, set cover is impossible.

That is:

```
// Number of arrays passed into the function: 2
// Number of elements in the union: 1
set_cover([1],[1]) // False
```

To turn that assertion into an algorithm, we begin by ordering the arrays by length. Then it is a simple matter of popping off the longest one and checking if the assertion holds. We repeat this process until there is one array left (that has at least one element).

When set cover is possible, pop elements off until only one array is left:

```
// Number of arrays: 3
// Length of Union: 3 [1,2,3]
[1],[2],[2,3] // Possible: True

// Pop off the longest one
// Number of arrays: 2
// Length of Union: 2 [1,2]
[1],[2] // Possible: True

// Pop off the longest one
// Number of arrays: 1
// Length of Union: 1
[1] // Possible: True
```

When set cover is *not* possible, the function returns early:

```
// Number of arrays: 3
// Length of Union: 3 [1,2,3]
[1],[1],[2,3] // Possible: True

// Pop off the longest one
// Number of arrays: 2
// Length of Union: 1 [1]
[1],[1] // Possible: False
```
