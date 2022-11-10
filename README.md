## Dynamic dispatch

Allows compiling generic functions that do not have generic parameters inside a library, specifying all the possible specializations for each generic type
This avoids recompiling all the generics for each crate that uses the functions, allowing better compile times.
