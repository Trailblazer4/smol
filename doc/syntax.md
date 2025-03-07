# Concrete syntax

Here is the concrete syntax for smol programs.  The grammar below is
deliberately already in LL(1), so there is no need for refactoring.

Assignment and arithmetic is in prefix notation (also known as Polish notation).
Polish notation is easy to parse, it is already in LL(1), and it doesn't need
parentheses.  That's why we use it for this tiny language.  Here are some examples:
- We write `+ 40 2` instead of `40 + 2`, and
- we write `:= x * 40 + 2 3` instead of `x := 40 * (2 + 3)`.

Here are some tokens' definitions using regexes:
- `num ::= [0-9]+`.  All numeric literals are decimal.
- `id ::= [a-zA-Z_][a-zA-Z0-9_]*`.

All whitespace and C++-style line comments are ignored.  The corresponding
regexes are:
- `whitespace ::= [ \t\v\n\r\f]+`
- `comment ::=//.*`
  - There is no `\n` at the end of `comment` so that a program without a
    trailing newline is still well-formed.

Meta-notation:
- All other tokens are wrapped in backticks (\`) in the grammar below.
- Meta-comments are added as C++-style comments.
- `α*` means 0 or more instances of α.
- `'*'` is a literal asterisk character.

```
// smol programs
program ::= stmt*

// statements
stmt ::= ':=' id expr      // assignment
       | '$print' expr
       | '$read' id
       | '$if' expr block block
       
block ::= '{' stmt* '}'

// expressions
expr ::= id              // variables
       | num             // numeric literals
       | bop expr expr   // binary operations
       | '~' expr        // negation
       
// binary operators
bop ::= '*' | '/' | '+' | '-' | '<'
```

## Example programs

Here is an example program that prints the maximum of two numbers:

```
$read a
$read b
if < a b {
  $print a
} {
  $print b
}
```

Notice that we don't need an `else` keyword because we always have a false branch.

Here is a program that calculates a given quadratic equation at the given point.
It checks whether `a` is 0, and exits early if that is the case:

```
$read a

if a {
} {
  $read b
  $read c
  $read x
  // this is  a * x * x + b * x + c in prefix notation
  $print + * a * x x + * b x c
}
```
