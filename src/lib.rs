//! The `droprate` crate aims to be a drop-in solution for picking options from
//! a weighted list of possibilities.
//! 
//! While naive random number generation to pick from some list of traits is
//! pretty easy to implement as-needed, it doesn't take long to run into scenarios
//! where this solution provides suboptimal results.
//! 
//! In a card game, you may want to simulate a deck of cards being shuffled, which
//! means that the odds of each card becomes zero once it's been pulled from the
//! deck.

use std::collections::HashMap;

extern crate rand;
use rand::Rng;

pub trait ProbabilityTable<T, R> {
    /// Add an option to the random table with the assigned weight value.
    /// 
    /// You can chain multiple `push(...)` calls together to save a little space.
    /// 
    /// The weight is stored as an `f64` value, so you don't _need_ the specify
    /// the odds in pure-integer ratios. Also, while you could use numbers to look
    /// like a "precent chance", it's important to remember that each items weight
    /// calculates odds against the entire table, and so if you add "more than 100%"
    /// then you will end up with each outcome's odds being less than you put in.
    /// 
    /// In other words, it's best to think of this like a recipe with ratios:
    /// 
    /// * 2 parts Option A
    /// * 5 parts Option B
    /// * 1 part Option C
    /// * 12 parts Option D
    /// * etc.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use droprate::{RandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// 
    /// let mut table = RandomTable::<&'static str, ThreadRng>::new(ThreadRng::default());
    /// table.push("First option", 1f64)  // 1/4 = 25% chance
    ///      .push("Second option", 1f64) // 1/4 = 25% chance
    ///      .push("Third option", 2f64); // 2/4 = 50% chance
    /// 
    /// assert_eq!(3, table.count());
    /// ```
    fn push(&mut self, ident: T, weight: f64) -> &mut dyn ProbabilityTable<T, R>;

    /// Get the number of possible items in the table.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use droprate::{RandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// 
    /// let mut table = RandomTable::<&'static str, ThreadRng>::new(ThreadRng::default());
    /// table.push("First option", 1f64);
    /// assert_eq!(1, table.count());
    /// 
    /// table.push("Second option", 1f64)
    ///      .push("Third option", 2f64);
    /// assert_eq!(3, table.count());
    /// ```
    fn count(&self) -> usize;

    /// Get a vector of all of the options in the list. There is no guarantee
    /// over the order in which items are returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use droprate::{RandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// 
    /// let mut table = RandomTable::<&'static str, ThreadRng>::new(ThreadRng::default());
    /// table.push("A", 1f64)
    ///      .push("B", 1f64);
    /// 
    /// assert_eq!(2, table.keys().len());
    /// ```
    fn keys(&self) -> Vec<T>;

    /// Choose an option from the list of options, using the supplied weights
    /// to map the odds. The actual odds for each trial may differ when using
    /// tables other than [`RandomTable`] (e.g., [`FairlyRandomTable`] tracks
    /// results from previous trials in order to deliver a more evenly distributed
    /// set of results than one would fine from a more "true" random generator)    
    /// 
    /// # Errors
    /// 
    /// If no options have been specified (or all options added were give a weight
    /// less than or equal to zero), this will return an error that the generated
    /// value was outside the range of the table.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use droprate::{RandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// 
    /// let mut table = RandomTable::<&'static str, ThreadRng>::new(ThreadRng::default());
    /// assert_eq!(true, table.random().is_err());
    /// 
    /// table.push("A", 1f64)
    ///      .push("B", 1f64);
    /// 
    /// assert_eq!(false, table.random().is_err());
    /// ```
    fn random(&mut self) -> Result<T, String>;

    //fn set_generator(rng: R);
}

/// `RandomTable` represents a table of options and their relative weights. The
/// odds of any option being selected is `option's weight / all options' weights`.
/// This is a typical implementation of random tables in software and games as
/// each trial runs independent of other trials which may have happened in the
/// past. As such, it is perfectly valid for an option with 50% odds to be
/// selected many times in a row. (It's an outcome which becomes statistically
/// less likely to have happened, but nevertheless if you have 5 in a row, the
/// next trial is still a 50% chance.)
/// 
/// This kind of random is useful, but it's also hard for users to understand
/// and can often lead to outcomes which (in games, at least) feel unfair.
pub struct RandomTable<T, R> {
    pub(crate) table: HashMap<T, f64>,
    pub(crate) total: f64,
    pub(crate) rng: R,
}

// RandomTable
impl<T: std::cmp::Eq + std::hash::Hash, R: Rng> RandomTable<T, R> {
    /// Create a new instance of `RandomTable` with no options.
    pub fn new(rng: R) -> RandomTable<T, R> {
        RandomTable {
            table: HashMap::new(),
            total: 0f64,
            rng,
        }
    }

    /// Create a new `RandomTable` from a [`HashMap`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use droprate::{RandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// use std::collections::HashMap;
    /// 
    /// let map: HashMap<&'static str, f64> =
    ///     [("A", 1f64),
    ///     ("B", 1f64),
    ///     ("C", 3f64)]
    ///     .iter().cloned().collect();
    /// 
    /// let mut table = RandomTable::<&'static str, ThreadRng>::from_map(map, ThreadRng::default());
    /// 
    /// assert_eq!(3, table.count());
    /// ```
    /// 
    /// ```
    /// use droprate::{RandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// use std::collections::HashMap;
    /// 
    /// let mut map = HashMap::new();
    /// map.insert("A", 1f64);
    /// map.insert("B", 1f64);
    /// map.insert("C", 3f64);
    /// 
    /// let mut table = RandomTable::<&'static str, ThreadRng>::from_map(map, ThreadRng::default());
    /// 
    /// assert_eq!(3, table.count());
    /// ```
    pub fn from_map(in_table: HashMap<T, f64>, rng: R) -> RandomTable<T, R> {
        let mut total = 0f64;
        for entry in &in_table {
            total += entry.1
        }

        RandomTable {
            table: in_table,
            total: total,
            rng,
        }
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + Clone, R: Rng> ProbabilityTable<T, R> for RandomTable<T, R> {
    fn push(&mut self, ident: T, weight: f64) -> &mut dyn ProbabilityTable<T, R> {
        self.table.insert(ident, weight);
        self.total += weight;
        self
    }

    fn count(&self) -> usize {
        self.table.len()
    }

    fn keys(&self) -> Vec<T> {
        self.table.keys().cloned().collect()
    }

    fn random(&mut self) -> Result<T, String> {
        let r = self.rng.gen::<f64>() * self.total;
        let mut comp = r;
        for pair in &self.table {
            if *pair.1 > comp {
                return Ok(pair.0.clone());
            }
            comp -= pair.1;
        }

        Err("Generated random outside of possible range".to_owned())
    }
}

/// `FairlyRandomTable` aims to create results which a human might create when
/// asked to create a random sequence from a weighted table. It is human nature
/// to generate a more evenly-distributed list of random values because we are
/// aware of the the history of options chosen.
/// 
/// Calling this a "random" table strains the definition -- it is certainly going
/// to give you "mixed up" results, but ultimately they will be far more
/// predictable. That said, they will _also_ feel much more "fair" to users who
/// experience them.
/// 
/// For example: Given a result with a 1-in-10 odds ratio, there is still a 30%
/// chance that you won't get that result within 10 trials. There is a 12% chance
/// that you won't even see the result within 20 trials. This is where players
/// start to complain that the devs hate them.
/// 
/// With `FairlyRandomTable`, every trial which doesn't give a certain result
/// increases the probability of that result on the next trial (proportional to
/// its initial probability) until it is selected, which decreases its probability
/// dramatically (however it's not impossible to get multiple results in a row --
/// in fact, allowing for multiple results in a row of even unlikely options is
/// a design goal; you just won't seem them as frequently).
pub struct FairlyRandomTable<T, R> {
    pub(crate) base: RandomTable<T, R>,
    pub(crate) table: HashMap<T, f64>,
    pub(crate) total: f64,
}

//
// FairlyRandomTable
//
impl<T: std::cmp::Eq + std::hash::Hash + Clone, R: Rng> FairlyRandomTable<T, R> {
    /// Create a new instance of `FairlyRandomTable` with no options.
    pub fn new(rng: R) -> FairlyRandomTable<T, R> {
        FairlyRandomTable {
            base: RandomTable::new(rng),
            table: HashMap::new(),
            total: 0f64,
        }
    }

    /// Create a new `FairlyRandomTable` from a [`HashMap`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use droprate::{FairlyRandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// use std::collections::HashMap;
    /// 
    /// let map: HashMap<&'static str, f64> =
    ///     [("A", 1f64),
    ///     ("B", 1f64),
    ///     ("C", 3f64)]
    ///     .iter().cloned().collect();
    /// 
    /// let mut table = FairlyRandomTable::<&'static str, ThreadRng>::from_map(map, ThreadRng::default());
    /// 
    /// assert_eq!(3, table.count());
    /// ```
    /// 
    /// ```
    /// use droprate::{FairlyRandomTable, ProbabilityTable};
    /// use rand::prelude::*;
    /// use std::collections::HashMap;
    /// 
    /// let mut map = HashMap::new();
    /// map.insert("A", 1f64);
    /// map.insert("B", 1f64);
    /// map.insert("C", 3f64);
    /// 
    /// let mut table = FairlyRandomTable::<&'static str, ThreadRng>::from_map(map, ThreadRng::default());
    /// 
    /// assert_eq!(3, table.count());
    /// ```
    pub fn from_map(in_table: HashMap<T, f64>, rng: R) -> FairlyRandomTable<T, R> {
        let mut total = 0f64;
        for entry in &in_table {
            total += entry.1
        }

        FairlyRandomTable {
            base: RandomTable::from_map(in_table.clone(), rng),
            table: in_table,
            total: total,
        }
    }

    /// Run a trial from this as though it were a [`RandomTable`]. The table's
    /// results memory will not be affected, and as such future results from
    /// calling `random()` will not account for this trial.
    pub fn pure_random(&self) -> Result<T, String> {
        let r = rand::random::<f64>() * self.total;
        let mut comp = r;
        for pair in &self.base.table {
            if *pair.1 > comp {
                return Ok(pair.0.clone());
            }
            comp -= pair.1;
        }

        Err("Generated random outside of possible range".to_owned())
    }

    fn redistribute_weights(&mut self, amount: f64) {
        let keys = self.table.keys().cloned().collect::<Vec<T>>();

        for key in keys {
            let original = match self.base.table.get(&key) {
                Some(val) => *val,
                None => continue,
            };

            let local = match self.table.get_mut(&key) {
                Some(val) => val,
                None => continue,
            };

            let ratio = original / self.base.total;

            *local += amount * ratio;
        }
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + Clone, R: Rng> ProbabilityTable<T, R> for FairlyRandomTable<T, R> {
    fn push(&mut self, ident: T, weight: f64) -> &mut dyn ProbabilityTable<T, R> {
        self.base.push(ident.clone(), weight);
        self.table.insert(ident, weight);
        self.total += weight;
        self
    }

    fn count(&self) -> usize {
        self.table.len()
    }

    fn keys(&self) -> Vec<T> {
        self.table.keys().cloned().collect()
    }

    fn random(&mut self) -> Result<T, String> {
        let r = rand::random::<f64>() * self.total;
        let mut comp = r;

        let keys = self.table.keys().cloned();
        let mut match_pair: Option<(T, f64)> = None;

        for key in keys {
            let val = self.table.get(&key);
            if let Some(val) = val {
                if *val > comp {
                    match_pair = Some((key.clone(), *val));
                    break;
                }
                comp -= *val;
            }
        }

        match match_pair {
            Some(pair) => {
                self.table.entry(pair.0.clone()).and_modify(|e| *e = 0f64);
                self.redistribute_weights(pair.1);
                return Ok(pair.0.clone());
            }
            None => {}
        }

        Err("Generated random outside of possible range".to_owned())
    }
}
