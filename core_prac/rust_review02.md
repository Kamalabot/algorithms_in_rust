{2024-11-03}

Session 1: {05:25}

Session 2: {07:49}

Q1:

- Rust provides memory safe code by assigning
  ownership of values to a single variable. The
  variable is automatically garbage collected as
  it goes out of scope.
- Since there is single owner reassignment moves
  or copies the value depending on the type.
- Rust ownership model avoids data races, and
  danglin references

Q2:

- There has to be only single owner
- The value can immutably or mutably assigned
- When the variable goes out of scope the value of
  the memory is dropped

Q3:

- Barrowing is a way to reference a value with
  another variable.
- Barrowing can be mutable or immutable
- Lifetimes notation ensures the variable lives
  the longest and doesn't leave any danglin
  references

Q4:

- Result type is a Enum with 2 variants Ok and Err
- Used by function to handle errors and error
  propagation
- Use unwrap, expect and ? keywords to manage the
  errors

Q5:

- Memory safety is achieved using the Ownership
  model, life times, and barrowing.

Q6:

- String is a heap allocated growable type

- &str is a immutable ref to a slice of string

Q7:

- Rust lifetimes annotations are represented using
  'a
- Used to denote the variable must live as long as
  the suggested life time.

Q8: Problem Solvin

- Write a factorial function with recursion with
  cache

Q9:

- Need to review the standard libraries.

- learnt about the std::{ops, fmt, iter, cmp,
  marker} modules that are used in creating trait
  bounds

Q10

- Send trait has to be implemented for the type
  which can be shared to different threads

- Sync trait has to be implemented for the type
  which is moved between threads

Q11

- Option is an Enum that indicates presence and
  absence of a value. Some(val) & None is used
  with match, if and while let constructs. It
  panics when unwrap() on None

Will update the rest of the questions after
completing the 2nd level interview questions
