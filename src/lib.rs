//! An immutable non-empty bytes container.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use core::num::NonZeroUsize;

/// A container to `T` that is known to be non-empty.
///
/// A `T: AsRef<[u8]>` is empty if `AsRef::<[u8]>::as_ref(&T).is_empty()` is `true`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonEmptyBz<T>(T);

impl<T> NonEmptyBz<T> {
	/// Creates a `NonEmptyBz<T>` if the given `bz` is not empty.
	///
	/// If `bz` is `[u8; N]` or `&[u8; N]`, consider using `NonEmptyBz::from_#_array()` constructor.
	pub fn new(bz: T) -> Option<Self>
	where
		T: AsRef<[u8]>,
	{
		(!bz.as_ref().is_empty()).then_some(Self(bz))
	}

	/// Returns reference to the contained value.
	pub const fn get(&self) -> &T {
		&self.0
	}

	/// Returns the first byte of the contained value.
	pub fn first(&self) -> u8
	where
		T: AsRef<[u8]>,
	{
		// SAFETY: `NonEmptyBz` guarantees that the contained value is non-empty i.e.
		// there is always at least one element at index 0.
		unsafe { *self.0.as_ref().get_unchecked(0) }
	}

	/// Returns the last byte of the contained value.
	pub fn last(&self) -> u8
	where
		T: AsRef<[u8]>,
	{
		let slice = self.0.as_ref();

		// SAFETY: `NonEmptyBz` guarantees that the contained value is non-empty,
		// so `len() >= 1` i.e. `len() - 1` is always a valid index:
		// - For len() == 1: len() - 1 == 0 (valid)
		// - For len() > 1: len() - 1 < len() (valid)
		// Therefore, `len() - 1` is always within bounds for a non-empty slice.
		unsafe { *slice.get_unchecked(slice.len() - 1) }
	}

	/// Returns the first byte and the remaining slice.
	///
	/// The remaining slice will be empty if the contained value has only one byte.
	pub fn split_first(&self) -> (u8, &[u8])
	where
		T: AsRef<[u8]>,
	{
		let first = self.first();

		// SAFETY: Since len() >= 1, the range [1..] is always valid.
		// When len() == 1, this creates an empty slice [1..1].
		// When len() > 1, this creates a valid slice [1..len()].
		let rest = unsafe { self.0.as_ref().get_unchecked(1..) };

		(first, rest)
	}

	/// Returns the last byte and the remaining slice.
	///
	/// The remaining slice will be empty if the contained value has only one byte.
	pub fn split_last(&self) -> (u8, &[u8])
	where
		T: AsRef<[u8]>,
	{
		let slice = self.0.as_ref();
		let len = slice.len();

		let last = self.last();

		// SAFETY: Since len() >= 1, the range [..len() - 1] is always valid.
		// When len() == 1, this creates an empty slice [..0].
		// When len() > 1, this creates a valid slice [..len() - 1].
		let rest = unsafe { slice.get_unchecked(..len - 1) };

		(last, rest)
	}

	/// Converts from `&NonEmptyBz<T>` to `NonEmptyBz<&T>`.
	pub const fn as_ref(&self) -> NonEmptyBz<&T> {
		NonEmptyBz(&self.0)
	}

	/// Returns [`NonEmptyBz`] containing the `AsRef::<[u8]>::as_ref` of the inner value.
	pub fn as_ref_slice(&self) -> NonEmptyBz<&[u8]>
	where
		T: AsRef<[u8]>,
	{
		NonEmptyBz(self.0.as_ref())
	}

	/// Returns the `len` of the inner value's `AsRef::<[u8]>::as_ref`.
	pub fn len(&self) -> NonZeroUsize
	where
		T: AsRef<[u8]>,
	{
		// SAFETY: `NonEmptyBz` guarantees that the contained value is non-empty,
		// i.e. `self.0.as_ref().len() >= 1`. This invariant is maintained by:
		// - The `NonEmptyBz::new()` constructor which rejects empty values
		// - The array constructors which use compile-time assertions to ensure non-empty array
		unsafe { NonZeroUsize::new_unchecked(self.0.as_ref().len()) }
	}

	/// Unwraps the value, consuming the `NonEmptyBz`.
	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<T> NonEmptyBz<&T> {
	/// Maps a `NonEmptyBz<&T>` to `NonEmptyBz<T>` by cloning the contained value.
	pub fn cloned(&self) -> NonEmptyBz<T>
	where
		T: Clone,
	{
		NonEmptyBz(self.0.clone())
	}
}

impl<const N: usize> NonEmptyBz<[u8; N]> {
	/// Creates a `NonEmptyBz<[u8; N]>` containing the array `bz`.
	///
	/// Will cause compile-time error if `N == 0`.
	pub const fn from_owned_array(bz: [u8; N]) -> Self {
		const { assert!(N != 0) }
		Self(bz)
	}
}

impl<'a, const N: usize> NonEmptyBz<&'a [u8; N]> {
	/// Creates a `NonEmptyBz<&[u8; N]>` containing the borrowed array `bz`.
	///
	/// Will cause compile-time error if `N == 0`.
	pub const fn from_borrowed_array(bz: &'a [u8; N]) -> Self {
		const { assert!(N != 0) }
		Self(bz)
	}
}

#[cfg(feature = "bytes")]
impl From<NonEmptyBz<&[u8]>> for NonEmptyBz<bytes::Bytes> {
	/// Creates a `NonEmptyBz<Bytes>` from a `NonEmptyBz<Bytes>` containing `&[u8]`
	/// by invoking [`Bytes::copy_from_slice`](bytes::Bytes::copy_from_slice).
	fn from(nebz_slice: NonEmptyBz<&[u8]>) -> Self {
		Self(bytes::Bytes::copy_from_slice(nebz_slice.get()))
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	// single byte
	#[case::single_byte_owned_array(NonEmptyBz::from_owned_array([42]), 42, 42, &[], &[])]
	#[case::single_byte_borrowed_array(NonEmptyBz::from_borrowed_array(b"a"), b'a', b'a', &[], &[])]
	#[case::single_byte_vec(NonEmptyBz::new(vec![0]).unwrap(), 0, 0, &[], &[])]
	#[case::single_byte_str(NonEmptyBz::new("z").unwrap(), b'z', b'z', &[], &[])]
	// multiple bytes
	#[case::multiple_bytes_owned_array(
		NonEmptyBz::from_owned_array([1, 2, 3, 4]),
		1,
		4,
		&[2, 3, 4],
		&[1, 2, 3],
	)]
	#[case::multiple_bytes_borrowed_array(
		NonEmptyBz::from_borrowed_array(b"nebz"),
		b'n',
		b'z',
		b"ebz",
		b"neb"
	)]
	#[case::multiple_bytes_vec(
		NonEmptyBz::new(vec![255, 0, 128]).unwrap(),
		255,
		128,
		&[0, 128],
		&[255, 0],
	)]
	#[case::multiple_bytes_str(
		NonEmptyBz::new("hello").unwrap(),
		b'h',
		b'o',
		b"ello",
		b"hell",
	)]
	fn first_last_works<T>(
		#[case] nebz: NonEmptyBz<T>,
		#[case] expected_first: u8,
		#[case] expected_last: u8,
		#[case] expected_split_first_rest: &[u8],
		#[case] expected_split_last_rest: &[u8],
	) where
		T: AsRef<[u8]>,
	{
		assert_eq!(nebz.first(), expected_first);
		assert_eq!(nebz.last(), expected_last);

		let (first, rest) = nebz.split_first();
		assert_eq!(first, expected_first);
		assert_eq!(rest, expected_split_first_rest);

		let (last, rest) = nebz.split_last();
		assert_eq!(last, expected_last);
		assert_eq!(rest, expected_split_last_rest);
	}

	#[rstest]
	// single byte
	#[case::single_byte_owned_array(NonEmptyBz::from_owned_array([42]), NonZeroUsize::MIN)]
	#[case::single_byte_borrowed_array(NonEmptyBz::from_borrowed_array(b"a"), NonZeroUsize::MIN)]
	#[case::single_byte_vec(NonEmptyBz::new(vec![0]).unwrap(), NonZeroUsize::MIN)]
	#[case::single_byte_str(NonEmptyBz::new("z").unwrap(), NonZeroUsize::MIN)]
	// multiple bytes
	#[case::multiple_bytes_owned_array(
		NonEmptyBz::from_owned_array([1, 2, 3, 4]),
		4.try_into().unwrap(),
	)]
	#[case::multiple_bytes_borrowed_array(
		NonEmptyBz::from_borrowed_array(b"nebz"),
		4.try_into().unwrap(),
	)]
	#[case::multiple_bytes_vec(NonEmptyBz::new(vec![255, 0, 128]).unwrap(), 3.try_into().unwrap())]
	#[case::multiple_bytes_str(NonEmptyBz::new("hello").unwrap(), 5.try_into().unwrap())]
	fn len_works<T>(#[case] nebz: NonEmptyBz<T>, #[case] expected_len: NonZeroUsize)
	where
		T: AsRef<[u8]>,
	{
		assert_eq!(nebz.len(), expected_len);
	}
}
