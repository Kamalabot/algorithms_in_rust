Ownership model:

A value is owned by a variable, and the moment the
variable goes out of scope the value is dropped
from the memory. There can be only one owner for
the value at a time. If the variable is assigned,
or used in function then the value is moved.

Ownership Rules:

There can be only owner for a value a time.

The value can be barrowed by another variable
mutably or immutably.

The value is assigned mutably or immutably

Once the source variable goes out of scope, rest
of the barrowed variables also loose the value

Q3: Result is a Enum with Ok and Err variant,
where the result of successful operation is
contained in the Ok variant, and the Err contains
the reason for the failure.

Q4: Life time annotation

Life time annotation are required to inform the
compiler that the life time of the variable from
which the value is barrowed lives long enough for
other barrowers can use the value.

Q5: String is a heap allocated variable that
contains the text data. &str is the slice of the
string which just references the original. This is
compulsorily immutable.

Q6: Send trait is implemented on a value that can
be shared between the threads. Sync is moved
between threads
