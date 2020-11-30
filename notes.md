Rust Journey
============

# Misc

- ownership has no overhead
- coherence, orphan rule ***

# Misc Tips

- `isize` or `usize` is when indexing some sort of collection
- debug mode compilation emits panics for overflows
- cannot return multiple types with `-> impl Trait` syntax
- grapheme clusters
- positional parameters inside `{}` belonging to format strings
- ^ or even named parameters    

# A-ha!s

- why in Java return type is not part of the signature -> because of polymorphism
- type mechanism ensures all references are valid when used

# Keywords

- `clone` method - deep copy
- `Copy` trait - for stack-only values, if `drop` is implemented then it cannot be `Copy`
- `drop` method - sort of destructor

## Traits
  
Trait bounds can be:
- specified for generic bounds either in `<>` or in `where` clauses
- ad-hoc as parameter types as `t: (impl T + P)`

## String

- there's `OsString` and `CString` besides the basic ones
- `char` in `Rust` is Unicore scalar values
- indexing a grapheme cluster takes linear time

## Format strings

- pretty printing with `"{:#?}"`

## Primitives

- `usize` and `isize` are pointer sized (platform dependent)