# Rust Set-Cover

## Input and Output

This program is a Clickhouse EUDF (Executable User Defined Function). As such, it takes input from stdin and outputs to stdout.

It expects Clickhouse's [Rowbinary](https://clickhouse.com/docs/en/interfaces/formats/#rowbinary) data in the format `Array(Array(UInt32))` (i.e. `[[1,2,3],[1,2]]`).

It returns boolean output in `UInt8` (either 0 or 1).

## What it Does

Parabible searches involve finding matching words in the same location (syntactical units). Words are matched on a variety of features such that a single word may match more than one set of constraints and be a valid return value for different search terms. For example, Parabible allows the user to search for singular nouns and masculine nouns. Of course the word "man" is both.

The problem is that for a query to be satisified, there should be some way of assigning a single matching word to each set of constraints. This means that, if the only matching word in a syntactical unit is "man", a search for syntactical units that contain "a singular noun" and "a masculine noun" should not return the unit. This is a type of "set cover" problem (though, not the typical one).

The problem may be understood by way of example:

```
set_cover([1,2],[1,2]) 		 = True
set_cover([1,2],[1]) 	     = True
set_cover([1],[1]) 			 = False
set_cover([1,2],[1,2,3]) 	 = True
set_cover([1,2],[1,2],[3])   = True
set_cover([1,2],[2,3],[3,1]) = True
set_cover([1],[1],[1,2,3])   = False
```

## How it Works
