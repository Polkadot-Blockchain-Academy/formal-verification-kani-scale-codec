#![allow(unused_imports)] // we want all of SCALE eventually ;)

// Import macros if derive feature is not used.
#[cfg(not(feature = "derive"))]
use parity_scale_codec_derive::{Decode, Encode};

use parity_scale_codec::{Decode, Encode};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn u32_encode_to_decode_single_value() {
		let x: u32 = 100;
		let encoded = x.encode();
		assert_eq!(u32::decode(&mut &encoded[..]).unwrap(), x);
	}
}

#[cfg(kani)]
#[kani::proof]
fn u32_encode_to_decode_all_values() {
	let x: u32 = kani::any();
	let encoded = x.encode();
	assert_eq!(u32::decode(&mut &encoded[..]).unwrap(), x);
}
