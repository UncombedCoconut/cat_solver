//! This is a stand alone crate that contains both the C source code of the
//! Kissat SAT solver together with its Rust binding. The C files are compiled
//! and statically linked during the build process.
//!
//! Kissat variants dominated the main track of the Sat Competition 2022.
//! Author Armin Biere describes Kissat as follows:
//!
//! Kissat is a "keep it simple and clean bare metal SAT solver" written in C.
//! It is a port of CaDiCaL back to C with improved data structures,
//! better scheduling of inprocessing and optimized algorithms and implementation.
//! Coincidentally "kissat" also means "cats" in Finnish.

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::fmt;

extern "C" {
    fn kissat_signature() -> *const c_char;
    fn kissat_init() -> *mut c_void;
    fn kissat_release(ptr: *mut c_void);
    fn kissat_add(ptr: *mut c_void, lit: c_int);
    fn kissat_solve(ptr: *mut c_void) -> c_int;
    fn kissat_value(ptr: *mut c_void, lit: c_int) -> c_int;
    fn kissat_reserve(ptr: *mut c_void, max_var: c_int);
    fn kissat_set_conflict_limit(ptr: *mut c_void, limit: c_uint);
    fn kissat_set_decision_limit(ptr: *mut c_void, limit: c_uint);
}

/// The Kissat SAT solver. The literals are unwrapped positive and negative integers,
/// as in the DIMACS format. The common IPASIR operations are presented in a safe Rust interface.
/// # Examples
/// ```
/// let mut sat: cat_solver::Solver = Default::default();
/// sat.add_clause([1, 2]);
/// sat.add_clause([-1, 2]);
/// assert_eq!(sat.solve(), Some(true));
/// assert_eq!(sat.value(2), Some(true));
/// ```

pub struct Solver {
    ptr: *mut c_void,
}

impl Solver {
    /// Constructs a new solver instance.
    pub fn new() -> Self {
        let ptr = unsafe { kissat_init() };
        Self { ptr }
    }

    /// Increases the maximum variable index explicitly.
    #[inline]
    pub fn reserve(&mut self, max_var: i32)
    {
        debug_assert!(max_var > 0);
        unsafe { kissat_reserve(self.ptr, max_var) };
    }

    /// Returns the name and version of the Kissat library.
    pub fn signature(&self) -> &str {
        let sig = unsafe { CStr::from_ptr(kissat_signature()) };
        sig.to_str().unwrap_or("invalid")
    }

    /// Adds the given clause to the solver. Negated literals are negative
    /// integers, positive literals are positive ones. All literals must be
    /// non-zero and different from `i32::MIN`.
    /// Beware: Kissat will abort if you try this after solve(),
    /// as incremental solving is not yet implemented.
    #[inline]
    pub fn add_clause<I>(&mut self, clause: I)
    where
        I: IntoIterator<Item = i32>,
    {
        for lit in clause {
            debug_assert!(lit != 0 && lit != std::i32::MIN);
            unsafe { kissat_add(self.ptr, lit) };
        }
        unsafe { kissat_add(self.ptr, 0) };
    }

    /// Solves the formula defined by the added clauses. If the formula is
    /// satisfiable, then `Some(true)` is returned. If the formula is
    /// unsatisfiable, then `Some(false)` is returned. If the solver runs out
    /// of resources or was terminated, then `None` is returned.
    /// Beware: Kissat will abort if you try this after solve(),
    /// as incremental solving is not yet implemented.
    pub fn solve(&mut self) -> Option<bool> {
        let r = unsafe { kissat_solve(self.ptr) };
        if r == 10 {
            Some(true)
        } else if r == 20 {
            Some(false)
        } else {
            None
        }
    }

    /// Returns the value of the given literal in the last solution. The
    /// state of the solver must be `Some(true)`. The returned value is
    /// `None` if the formula is satisfied regardless of the value of the
    /// literal.
    #[inline]
    pub fn value(&self, lit: i32) -> Option<bool> {
        debug_assert!(lit != 0 && lit != std::i32::MIN);
        let val = unsafe { kissat_value(self.ptr, lit) };
        if val == lit {
            Some(true)
        } else if val == -lit {
            Some(false)
        } else {
            None
        }
    }

    /// Sets a solver limit with the corresponding name to the given value.
    /// These limits are only valid for the next `solve` call
    /// and reset to their default values, which disables them.
    /// The following limits are supported:
    /// * `conflicts`: max conflicts detected before the solver aborts.
    /// * `decisions`: max decisions made before the solver aborts.
    pub fn set_limit<S: AsRef<str>>(&mut self, name: S, limit: u32) -> Result<(), Error> {
        match name.as_ref() {
            "conflicts" => unsafe { kissat_set_conflict_limit(self.ptr, limit) },
            "decisions" => unsafe { kissat_set_decision_limit(self.ptr, limit) },
            _ => return Err(Error::new("unknown limit")),
        };
        Ok(())
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver::new()
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { kissat_release(self.ptr) };
    }
}

/// Kissat does not use thread local variables, so it is possible to
/// move it between threads. However it cannot be used queried concurrently
/// (for example getting the value from multiple threads at once), so we
/// do not implement `Sync`.
unsafe impl Send for Solver {}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Error type for configuration errors.
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.msg.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn solver() {
        let mut sat: Solver = Solver::new();
        assert!(sat.signature().starts_with("kissat-"));
        sat.add_clause([1, 2]);
        assert_eq!(sat.solve(), Some(true));
        sat = Solver::new();
        sat.add_clause([1, 2]);
        sat.add_clause([-1]);
        sat.add_clause([-2]);
        assert_eq!(sat.solve(), Some(false));
    }

    fn pigeon_hole(num: i32) -> Solver {
        let mut sat: Solver = Solver::new();
        for i in 0..(num + 1) {
            sat.add_clause((0..num).map(|j| 1 + i * num + j));
        }
        for i1 in 0..(num + 1) {
            for i2 in 0..(num + 1) {
                if i1 == i2 {
                    continue;
                }
                for j in 0..num {
                    let l1 = 1 + i1 * num + j;
                    let l2 = 1 + i2 * num + j;
                    sat.add_clause([-l1, -l2])
                }
            }
        }
        sat
    }

    #[test]
    fn decision_limit() {
        let mut sat = pigeon_hole(5);
        sat.set_limit("decisions", 100).unwrap();
        let result = sat.solve();
        assert_eq!(result, None);
    }

    #[test]
    fn conflict_limit() {
        let mut sat = pigeon_hole(5);
        sat.set_limit("conflicts", 100).unwrap();
        let result = sat.solve();
        assert_eq!(result, None);
    }

    #[test]
    fn bad_limit() {
        let mut sat = pigeon_hole(5);
        assert!(sat.set_limit("bad", 0) == Err(Error::new("unknown limit")));
    }

    #[test]
    fn moving() {
        let mut sat = pigeon_hole(5);
        let id = thread::spawn(move || {
            assert_eq!(sat.solve(), Some(false));
        });
        id.join().unwrap();
    }
}
