// This file is Copyright its original authors, visible in version control history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. You may not use this file except in
// accordance with one or both of these licenses.

use rand::Rng as _;

/// Returns a random `u64` uniformly distributed in `[min, max]` (inclusive).
pub(crate) fn random_range(min: u64, max: u64) -> u64 {
	debug_assert!(min <= max);
	rand::rng().random_range(min..=max)
}
