/// Knuth's optimal 64-bit multiplier (a) for a LCG, used in MMIX and Newlib.
const KNUTH_A: i64 = 6364136223846793005;

/// Knuth's optimal 64-bit increment (c) for a LCG, used in MMIX.
const KNUTH_C: i64 = 1442695040888963407;

/// Steps a LCG using Knuth's optimal A and C constants, and with a modulus of 2^64.
fn step_knuth(state: i64) -> i64 {
	state.wrapping_mul(KNUTH_A).wrapping_add(KNUTH_C)
}

/// Steps a LCG with an A value determined by the output of the step_knuth function and a C value provided by the caller. Modulus is still 2^64.
fn step_salted(state: i64, c: i64) -> i64 {
	state.wrapping_mul(step_knuth(state)).wrapping_add(c)
}

/// Notch's custom RNG, that can be initialized from a position.
/// Used commonly in Beta 1.8 and later worldgen for biome generation, among other things.
#[derive(Debug, Clone)]
pub struct NotchRng {
	/// Initial value assigned to the RNG at a position before mixing in the coordinates.
	pub initial: i64,
	/// The current internal state of the RNG, initialized using `NotchRng::init_at` and modified with the next... functions.
	pub state:   i64
}

impl NotchRng {
	/// Initialize a NotchRng. The seed value usually represents the world seed.
	/// The salt is a unique value that differentiates this RNG from other instances with the same world seed.
	pub fn new(salt: i64, seed: i64) -> Self {
		let mut primary = salt;

		primary = step_salted(primary, salt);
		primary = step_salted(primary, salt);
		primary = step_salted(primary, salt);

		let mut initial = seed;

		initial = step_salted(initial, primary);
		initial = step_salted(initial, primary);
		initial = step_salted(initial, primary);

		NotchRng {
			initial,
			state: 0
		}
	}

	pub fn init_at(&mut self, x: i64, z: i64) {
		self.state = self.initial;

		self.state = step_salted(self.state, x);
		self.state = step_salted(self.state, z);
		self.state = step_salted(self.state, x);
		self.state = step_salted(self.state, z);
	}

	/// Steps the RNG forward by one. Unlike JavaRng, this function always returns 40 bits.
	pub fn next(&mut self) -> i64 {
		let result = self.state >> 24;

		self.state = step_salted(self.state, self.initial);

		result
	}

	/// Returns an i32 in the range [0, max).
	/// Make sure to call `init_at(x, z)` first if calling this from world generation code!
	pub fn next_i32(&mut self, max: i32) -> i32 {
		if max <= 0 {
			panic!("Maximum must be > 0")
		}

		// Get a value in the range (-max, max)
		let result = (self.next().wrapping_rem(max as i64)) as i32; // TODO: Casting order

		// Shift the result into the range [0, max)
		result + if result < 0 { max } else { 0 }
	}
}