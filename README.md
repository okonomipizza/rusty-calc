# Rusty Calculator

A simple, interactive calculator in Rust.
This project implements a lexer and a parser to evaluate basic arithmetic expressions in the command line interface (CLI).
It supports integers and floating-point numbers alog with a variety of operators.

Features
- Integer and Float Values: Perform calculations using both integers and floating-point numbers.
- Arithmetic Operations: Basic arithmetic such as `+`,  `-`,  `*`,  `/`,  `%`
- Boolean Values: Evaluate boolean expressions with true and falses.
- Conditional Operators: Support for comparison operations such as `==`,  `<`,  `<=`,  `>`  and  `>==`.

## Examples

You can use the Rusty Calculator in your CLI as follows:
1. Lauch the program: Run the calculator from the terminal.
```
$ cargo run
```
2. Input Formula (Do not forget type `;;` at end of your input)
```
1#  1 + 2 ;;
- : int = 3
```
3. Exit: Type `exit` to quit the program
```
1# exit ;;
Process will be finished
```
