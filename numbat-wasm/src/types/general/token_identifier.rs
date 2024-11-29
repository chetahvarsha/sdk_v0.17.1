use super::BoxedBytes;
use crate::abi::TypeAbi;
use alloc::string::String;
use numbat_codec::*;

/// Specialized type for handling token identifiers.
/// It wraps a BoxedBytes with the full ASCII name of the token.
/// REWA is stored as an empty name.
///
/// Not yet implemented, but we might add additional restrictions when deserializing as argument.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenIdentifier(BoxedBytes);

impl TokenIdentifier {
	/// This special representation is interpreted as the REWA token.
	pub const REWA_REPRESENTATION: &'static [u8] = b"REWA";

	pub const TICKER_MIN_LENGTH: usize = 3;
	pub const TICKER_MAX_LENGTH: usize = 10;
	pub const ADDITIONAL_RANDOM_CHARS_LENGTH: usize = 6;
	// +1 because of the '-' (dash) between ticker and the random chars
	pub const IDENTIFIER_MIN_LENGTH: usize =
		Self::TICKER_MIN_LENGTH + Self::ADDITIONAL_RANDOM_CHARS_LENGTH + 1;
	pub const IDENTIFIER_MAX_LENGTH: usize =
		Self::TICKER_MAX_LENGTH + Self::ADDITIONAL_RANDOM_CHARS_LENGTH + 1;

	pub const DASH_CHARACTER: u8 = b'-';

	/// New instance of the special REWA token representation.
	pub fn rewa() -> Self {
		TokenIdentifier(BoxedBytes::empty())
	}

	#[inline]
	pub fn is_rewa(&self) -> bool {
		self.is_empty()
	}

	#[inline]
	pub fn is_dcdt(&self) -> bool {
		!self.is_rewa()
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn into_boxed_bytes(self) -> BoxedBytes {
		self.0
	}

	#[deprecated(
		note = "Please use the as_dcdt_identifier method instead, its name is more suggestive."
	)]
	#[inline]
	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}

	#[inline]
	pub fn as_dcdt_identifier(&self) -> &[u8] {
		self.0.as_slice()
	}

	#[inline]
	pub fn as_ptr(&self) -> *const u8 {
		self.0.as_ptr()
	}

	#[inline]
	pub fn as_name(&self) -> &[u8] {
		if self.is_rewa() {
			TokenIdentifier::REWA_REPRESENTATION
		} else {
			self.0.as_slice()
		}
	}

	pub fn is_valid_dcdt_identifier(&self) -> bool {
		if self.is_rewa() {
			return false;
		}

		let id_len = self.0.len();
		#[allow(clippy::manual_range_contains)]
		if id_len < Self::IDENTIFIER_MIN_LENGTH || id_len > Self::IDENTIFIER_MAX_LENGTH {
			return false;
		}

		let id_as_slice = self.0.as_slice();

		// ticker must be all uppercase
		let ticker_len = id_len - Self::ADDITIONAL_RANDOM_CHARS_LENGTH - 1;
		let ticker = &id_as_slice[..ticker_len];
		for ticker_char in ticker {
			let is_uppercase_letter = (&b'A'..=&b'Z').contains(&ticker_char);
			if !is_uppercase_letter {
				return false;
			}
		}

		let dash_position = ticker_len;
		if id_as_slice[dash_position] != Self::DASH_CHARACTER {
			return false;
		}

		// random chars are alphanumeric lowercase
		let random_chars = &id_as_slice[(id_len - Self::ADDITIONAL_RANDOM_CHARS_LENGTH)..];
		for rand_char in random_chars {
			let is_lowercase_letter = (&b'a'..=&b'z').contains(&rand_char);
			let is_number = (&b'0'..=&b'9').contains(&rand_char);
			if !is_lowercase_letter && !is_number {
				return false;
			}
		}

		true
	}
}

impl AsRef<[u8]> for TokenIdentifier {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.0.as_ref()
	}
}

impl From<BoxedBytes> for TokenIdentifier {
	#[inline]
	fn from(boxed_bytes: BoxedBytes) -> Self {
		if boxed_bytes.as_slice() == TokenIdentifier::REWA_REPRESENTATION {
			TokenIdentifier::rewa()
		} else {
			TokenIdentifier(boxed_bytes)
		}
	}
}

impl<'a> From<&'a [u8]> for TokenIdentifier {
	#[inline]
	fn from(byte_slice: &'a [u8]) -> Self {
		if byte_slice == TokenIdentifier::REWA_REPRESENTATION {
			TokenIdentifier::rewa()
		} else {
			TokenIdentifier(BoxedBytes::from(byte_slice))
		}
	}
}

impl PartialEq<&[u8]> for TokenIdentifier {
	#[inline]
	fn eq(&self, other: &&[u8]) -> bool {
		if self.is_rewa() {
			*other == TokenIdentifier::REWA_REPRESENTATION
		} else {
			self.0.as_slice().eq(*other)
		}
	}
}

impl NestedEncode for TokenIdentifier {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_name().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_name().dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for TokenIdentifier {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_name().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_name().top_encode_or_exit(output, c, exit);
	}
}

impl NestedDecode for TokenIdentifier {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(TokenIdentifier::from(BoxedBytes::dep_decode(input)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		TokenIdentifier::from(BoxedBytes::dep_decode_or_exit(input, c, exit))
	}
}

impl TopDecode for TokenIdentifier {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(TokenIdentifier::from(BoxedBytes::top_decode(input)?))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		TokenIdentifier::from(BoxedBytes::top_decode_or_exit(input, c, exit))
	}
}

impl TypeAbi for TokenIdentifier {
	fn type_name() -> String {
		"TokenIdentifier".into()
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::*;
	use numbat_codec::test_util::*;

	#[test]
	fn test_rewa() {
		assert!(TokenIdentifier::rewa().is_rewa());
	}

	#[test]
	fn test_codec() {
		check_top_encode_decode(
			TokenIdentifier::rewa(),
			TokenIdentifier::REWA_REPRESENTATION,
		);
		check_dep_encode_decode(
			TokenIdentifier::rewa(),
			dep_encode_to_vec_or_panic(&TokenIdentifier::REWA_REPRESENTATION).as_slice(),
		);

		// also allowed
		assert_eq!(
			TokenIdentifier::rewa(),
			check_top_decode::<TokenIdentifier>(&[])
		);
		assert_eq!(
			TokenIdentifier::rewa(),
			check_dep_decode::<TokenIdentifier>(&[0, 0, 0, 0])
		);
	}

	#[test]
	fn test_is_valid_dcdt_identifier() {
		// valid identifier
		assert!(TokenIdentifier::from(&b"ALC-6258d2"[..]).is_valid_dcdt_identifier());

		// missing dash
		assert!(!TokenIdentifier::from(&b"ALC6258d2"[..]).is_valid_dcdt_identifier());

		// wrong dash position
		assert!(!TokenIdentifier::from(&b"AL-C6258d2"[..]).is_valid_dcdt_identifier());

		// lowercase ticker
		assert!(!TokenIdentifier::from(&b"alc-6258d2"[..]).is_valid_dcdt_identifier());

		// uppercase random chars
		assert!(!TokenIdentifier::from(&b"ALC-6258D2"[..]).is_valid_dcdt_identifier());

		// too many random chars
		assert!(!TokenIdentifier::from(&b"ALC-6258d2ff"[..]).is_valid_dcdt_identifier());

		// ticker too short
		assert!(!TokenIdentifier::from(&b"AL-6258d2"[..]).is_valid_dcdt_identifier());

		// ticker too long
		assert!(!TokenIdentifier::from(&b"ALCCCCCCCCC-6258d2"[..]).is_valid_dcdt_identifier());
	}
}
