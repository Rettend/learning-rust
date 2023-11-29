use crate::garden::vegetables::Asparagus;
pub mod garden;

use std::collections::HashMap;

use std::fmt;
use std::io;

use rand::Rng;

// Nested paths:
// use std::cmp::Ordering;
// use std::io;
use std::{any, cmp::Ordering};

// Nested paths with self:
// use std::mem;
// use std::mem::replace;
use std::mem::{self, replace};

// Glob operator:
use std::collections::*;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    // it's ok to import HashMap and not it's parent module: collections
    let mut map = HashMap::new();
    map.insert(1, 2);

    // we need to import the parent modules
    // otherwise the Result type is ambiguous
    fn function1() -> fmt::Result {
        Ok(())
    }

    fn function2() -> io::Result<()> {
        Ok(())
    }

    // we can also use the as keyword to rename the import
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function3() -> Result {
        Ok(())
    }

    fn function4() -> IoResult<()> {
        Ok(())
    }

    let secret_number = rand::thread_rng().gen_range(1..=100);
}
