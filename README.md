# CALC

This is a simple CLI script to evaluate math expression.

## Usage

You can pass in a math expression into the program. The result is printed to stdout.

There are 3 ways to pass in the expressions:

1. Directly as command line arguments:\
   `calc 3+4(6*2) 5+6`
2. You can specify a file. The filename is set by the -f or --file flags:\
   `calc --file math_expressions.txt`\
   In the file the single expressions must be separated by any form of whitespace.
3. The expressions can be passed into stdin, when no other arguments are supplied.\
   `echo "4-3*2" | calc`

### Internals

The result is evaluated as a 64bit float.

If you want to use this code in you project, you can pass in any type that implements the [ParsableNumber](src/number.rs) trait.
But keep in mind, that the operations of the specific type are used for addition, subtraction, multiplication and division.
Therefore, integers use integer divisions and are rounded down.

## Supported Syntax

Any whitespace is interpreted as a divider between expressions:

    "5+23 4*6" => "28 24"

These elements can be used to construct the expressions:

> Numbers: 6 and 6.5
> 
> Negatives: -6 and -(4+1)
> 
> Additions: 4+1
> 
> Subtractions: 5-3
> 
> Multiplication: 4*2 or 4x2
> 
> Division: 6/4
> 
> Parenthesis: (2+4) or [2+4]

## Architecture

The parsing of the expression is done with [nom](https://crates.io/crates/nom).
The evaluation is done without constructing an AST.
Therefore, no heap allocations besides the initial string are needed.
