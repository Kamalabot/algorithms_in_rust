{2024-11-02}

Morning Interview Practice::

- Rust ownership & barrow model

  - Each value has single owner
  - The value can be mutable or immutable
  - The value can be barrowed without taking
    ownership
  - There can be one mutable barrow of the value
    to avoid data races
  - Once the variable goes out of scope the value
    is dropped
  - Garbage collection is automatic in Rust

- Describe Result type:

  - Its an Enum with Ok() and Err() variants
  - Used when error handling needs to be
    implemented
  - Used to propagate the error to the calling
    function
  - Value is extracted with match, unwrap & expect
  - Both unwrap and expect will panic, so need to
    avoid in production code

- Rust memory safety:

  - Each value has single owner and the memory is
    freed when the owner goes out of scope
  - Each value is bound to a lifetime, the barrow
    checker checks if the value lives atleast for
    the longest life time
  - Depending on whether the value is Heap
    Allocated or stack allocated, the Copy trait
    is implemented. Heap allocated values are
    moved when reassigned.
  - When the value is barrowed, the barrowed
    variable should have life time lesser than
    owner else reference dangling will occur

- Differentiate String and &str:

  - String is heap allocated growable type
  - &str is an immutable slice of the given string
  - As &str is not mutable, and it is not owned
  - It can still be barrowed

- Why barrow a value?

  - To read the value without creating a clone of
    the value
  - Keep the reference to the owner, so when owner
    changes the barrowe value also changes

- Explain Rust LifeTime annotation's and their
  role

  - The variables are assigned life times when
    they are declared
  - !! Life time syntax needs to be practiced

```rust
fn longest<T 'a>(str1: T 'a, str2: T 'b){
    # check length of str1 > str2
    if str1.len() > str2.len() {
        return str1
    } else {
        return str2
    }
}
```

- Describe Options:

  - Option is an Enum with Some and None variant
  - Used when the value might not have a value
    assigned

- What is purpose of Arc & Mutex?

  - Atomic ref counter keeps count of number of
    reference created on a value
  - Ensure the owner doesn't go out of scope
    before all the references to the value are
    dropped
  - Used in multi-thread communication
  - Mutex is Mutually Exclusive value that can be
    modified by only one thread at a time. The
    thread which wants to modify needs get the
    lock() on the mutex

- What is purpose of Rc and RefCell?

  - Rc is Ref counted pointer, that keep tracks of
    refs/ barrows in non-threaded code.
  - RefCell allows the barrow and mutable barrow
    of Rc pointer values in non-threaded code.

- Recollect the Send and Sync purpose in relation
  to threads
