Kissat SAT solver
==================
[![GitHub](https://img.shields.io/github/license/UncombedCoconut/kissat-rs)](LICENSE)

This is a stand alone crate that contains both the C source code of the
Kissat SAT solver together with its Rust binding. The C files are compiled
and statically linked during the build process.

Kissat dominated the main track of the Sat Competition 2022.
It was written by Armin Biere, and it is available under the MIT license.
This crate is based on the "cadical" crate. (Kissat is the CaDiCaL author's
port "back to C with improved data structures, better scheduling of
inprocessing and optimized algorithms and implementation." Unlike its
predecessor, it does not yet support incremental solving, yet.)

The literals are unwrapped positive and negative integers, exactly as in the
DIMACS format. The common IPASIR operations are presented in a safe Rust
interface.

```
let mut sat: kissat::Solver = Default::default();
sat.add_clause([1, 2]);
sat.add_clause([-1, 2]);
assert_eq!(sat.solve(), Some(true));
assert_eq!(sat.value(2), Some(true));
```
