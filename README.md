Kissat SAT solver
==================
[![Crate](https://img.shields.io/crates/v/cat_solver)](https://crates.io/crates/cat_solver)
[![Documentation](https://docs.rs/cat_solver/badge.svg)](https://docs.rs/cat_solver)
[![GitHub](https://img.shields.io/github/license/UncombedCoconut/cat_solver)](LICENSE)

This is a stand alone crate that contains both the C source code of the
Kissat SAT solver together with its Rust binding. The C files are compiled
and statically linked during the build process.

Kissat variants dominated the main track of the Sat Competition 2022.
Author Armin Biere describes Kissat as follows:

Kissat is a "keep it simple and clean bare metal SAT solver" written in C.
It is a port of CaDiCaL back to C with improved data structures,
better scheduling of inprocessing and optimized algorithms and implementation.
Coincidentally "kissat" also means "cats" in Finnish.

This crate is based on the "cadical" crate, and is as API-compatible as possible.
This enables a switch back to `cadical::Solver` (which has extra features like file I/O)
as a debugging strategy.
Beware: this also means the API will let you try to modify the problem after solving,
but Kissat will abort if you do. Incremental solving is not yet implemented.

The literals are unwrapped positive and negative integers, exactly as in the
DIMACS format. The common IPASIR operations are presented in a safe Rust
interface.

```
let mut sat: kissat::Solver::new();
sat.add_clause([1, 2]);
sat.add_clause([-1, 2]);
assert_eq!(sat.solve(), Some(true));
assert_eq!(sat.value(2), Some(true));
```
