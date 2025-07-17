//! An immutable non-empty bytes container.

/// A container to `T` that is known to be non-empty.
///
/// A `T: AsRef<[u8]>` is empty if `AsRef::<[u8]>::as_ref(&T).is_empty()` is `true`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonEmptyBz<T>(T);

impl<T> NonEmptyBz<T> {
	/// Creates a [`NonEmptyBz`] if the given `bz` is not empty.
	///
	/// If 'bz' is `[u8; N]` or `&[u8; N]`, consider using `NonEmptyBz::from_#_array()` constructors.
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

	/// Converts from `&NonEmptyBz<T>` to `NonEmptyBz<&T>`.
	pub const fn as_ref(&self) -> NonEmptyBz<&T> {
		NonEmptyBz(&self.0)
	}

	/// Returns [`NonEmptyBz`] containing the [`AsRef::<[u8]>::as_ref`] of the inner value.
	pub fn as_ref_slice(&self) -> NonEmptyBz<&[u8]>
	where
		T: AsRef<[u8]>,
	{
		NonEmptyBz(self.0.as_ref())
	}

	/// Returns the `len` of the inner value's [`AsRef::<[u8]>::as_ref`].
	pub fn len(&self) -> usize
	where
		T: AsRef<[u8]>,
	{
		self.0.as_ref().len()
	}

	/// Returns true if the `len` of the inner value's [`AsRef::<[u8]>::as_ref`] is 0.
	pub fn is_empty(&self) -> bool
	where
		T: AsRef<[u8]>,
	{
		self.0.as_ref().is_empty()
	}

	/// Unwraps the value, consuming the [`NonEmptyBz`].
	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<const N: usize> NonEmptyBz<[u8; N]> {
	/// Creates a [`NonEmptyBz`] containing the array `bz`.
	///
	/// Will cause compile-time error if `N` is 0.
	pub const fn from_owned_array(bz: [u8; N]) -> Self {
		const { assert!(N != 0) }
		Self(bz)
	}
}

impl<'a, const N: usize> NonEmptyBz<&'a [u8; N]> {
	/// Creates a [`NonEmptyBz`] containing the borrowed array `bz`.
	///
	/// Will cause compile-time error if `N` is 0.
	pub const fn from_borrowed_array(bz: &'a [u8; N]) -> Self {
		const { assert!(N != 0) }
		Self(bz)
	}
}

#[cfg(feature = "bytes")]
impl From<NonEmptyBz<&[u8]>> for NonEmptyBz<bytes::Bytes> {
	/// Creates a [`NonEmptyBz`] containing `Bytes` from a [`NonEmptyBz`] containing `&[u8]` by invoking [`Bytes::copy_from_slice`](bytes::Bytes::copy_from_slice).
	fn from(nebz_slice: NonEmptyBz<&[u8]>) -> Self {
		Self(bytes::Bytes::copy_from_slice(nebz_slice.get()))
	}
}
