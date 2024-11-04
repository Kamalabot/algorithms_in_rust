{2024-11-04}

Start : {08:57} current: {09:50}

Start: {9:52} current : {10:24}

Start: {10:50} current:

Q1: Rust Ownership

- Every value is assigned one variable as owner,
  in other words there can be only one owner for a
  value.
- The variable will be automatically garbage
  collected when it goes out of scope
- Reassigning the value will move it to a
  different owner

Q3: Barrowin in Rust

- Barrowing is the concept of referring to the
  address of the variable.

- Barrowing can be done mutably and immutably

- When Barrowed, need to make sure the source
  variable doesn't go out of scope

Q4: Result in Rust

- Result is an Enum with 2 variants, Ok & Err.

- Used to handle error in rust gracefully.

- Unwrap, expect and ? are used for getting the
  value inside the Ok and panics when called on
  Err

Q5: Memory Safety

- Mem Safety is achieved through barrowing,
  lifetimes and ownership.

- Rust ownership rules allow only one owner at
  time

- When the owner goes out of scope, the memory
  allokated is freed by deallocator

- The variable referencing the source variable
  have to be within the scope, else compiler
  throws error

Q6: String Vs &str

String is a Growable heap memory allocated data
structure, which can be moved, referenced and
cloned.

&str is immutable slice of a string that is used
as reference.

Q7: Lifetime Annotation

Rust lifetimes are annotated usin 'a syntax. It is
used as a constraint when referencing the
variables, as the life time of the variable must
live long enough for the code to work.

lifetimes in rust ensures references are valid as
long as required, but not longer. life time inform
compiler what is the scope of the references, so
there are no dangling references and memory safety
is ensured

Q8: Concurrency in Rust

Rust manages concurrency by ownership and
barrowing rules for the variables that are
transmitted between threads.

Send trait implemented types allow for variables
to be moved between threads

Sync trait implemented traits allow for variable
to be shared between threads

Q9: Option in rust an enum with 2 variants that
describes presence and absence of a value. Some
and None are the two variants. Unwrap, expect can
be used for extracting the data within the
variants. Panics when unwrapped on None.

Q10: Rc and RefCell

Rc is ref counted pointer, it must count the
number of ownership a variable is having. It
counts the owners. What is the use of having
ownership if you cannot mutate it?

RefCell provides multiple ownership with interior
mutablilty with barrow checking at runtime

Rc<RefCell<>> provide shared ownership with
interior mutability

Review the thread communication examples
1730626517-FQOK

- Producer Consumer problem

- Dining philosopher problem

- Reader- Writers problem

- Barrier sync problem

- multiple producers and consumers with single
  buffer

Work on Pattern matching with Enums
