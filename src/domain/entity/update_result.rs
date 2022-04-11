//! UpdateResult values.
//!
//! Type [`UpdateResult`] represents an optional value: every [`UpdateResult`]
//! is either [`Created`] or [`Updated`], and contains a value.

/// The `UpdateResult` type. See [the module level documentation](self) for
/// more.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum UpdateResult<T> {
    Created(T),
    Updated(T),
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl<T> UpdateResult<T> {
    /// Returns the contained [`Created`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is dedicated for integration
    /// tests only.
    #[cfg(feature = "integration-tests")]
    #[inline]
    #[allow(dead_code)]
    pub fn unwrap_created(self) -> T {
        match self {
            UpdateResult::Created(item) => item,
            UpdateResult::Updated(_) => panic!("called `UpdateResult::unwrap_created()` on an `Updated` value"),
        }
    }

    /// Returns the contained [`Updated`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is dedicated for integration
    /// tests only.
    #[cfg(feature = "integration-tests")]
    #[inline]
    #[allow(dead_code)]
    pub fn unwrap_updated(self) -> T {
        match self {
            UpdateResult::Created(_) => panic!("called `UpdateResult::unwrap_updated()` on an `Created` value"),
            UpdateResult::Updated(item) => item,
        }
    }

    /// Returns the contained [`Created`] or [`Updated`] value, consuming the
    /// `self` value. # Examples
    ///
    /// ```
    /// use auth_config::domain::entity::UpdateResult;
    ///
    /// let x = UpdateResult::Created("created instance");
    /// assert_eq!(x.unwrap(), "created instance");
    ///
    /// let x = UpdateResult::Updated("Updated instance");
    /// assert_eq!(x.unwrap(), "Updated instance");
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub fn unwrap(self) -> T {
        match self {
            UpdateResult::Created(item) => item,
            UpdateResult::Updated(item) => item,
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the result is a [`UpdateResult::Created`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use auth_config::domain::entity::UpdateResult;
    ///
    /// let x = UpdateResult::Created(2);
    /// assert_eq!(x.is_created(), true);
    ///
    /// let x = UpdateResult::Updated(5);
    /// assert_eq!(x.is_created(), false);
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub const fn is_created(&self) -> bool {
        matches!(*self, UpdateResult::Created(_))
    }

    /// Returns `true` if the result is a [`UpdateResult::Updated`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use auth_config::domain::entity::UpdateResult;
    ///
    /// let x = UpdateResult::Updated(2);
    /// assert_eq!(x.is_updated(), true);
    ///
    /// let x = UpdateResult::Created(5);
    /// assert_eq!(x.is_updated(), false);
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub const fn is_updated(&self) -> bool {
        matches!(*self, UpdateResult::Updated(_))
    }

    /// Maps an `UpdateResult<T>` to `UpdateResult<U>` by applying a function to
    /// a contained value.
    ///
    /// # Examples
    ///
    /// Converts an <code>UpdateResult<[String]></code> into an
    /// <code>UpdateResult<[usize]></code>, consuming the original:
    ///
    /// [String]: ../../std/string/struct.String.html "String"
    /// ```
    /// use auth_config::domain::entity::UpdateResult;
    ///
    /// let maybe_Updated_string = UpdateResult::Updated(String::from("Hello, World!"));
    ///
    /// // `UpdateResult::map` takes self *by value*, consuming `maybe_Updated_string`
    /// let maybe_Updated_string_len = maybe_Updated_string.map(|s| s.len());
    ///
    /// assert_eq!(maybe_Updated_string_len, UpdateResult::Updated(13));
    #[inline]
    #[allow(dead_code)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> UpdateResult<U> {
        match self {
            UpdateResult::Created(x) => UpdateResult::Created(f(x)),
            UpdateResult::Updated(x) => UpdateResult::Updated(f(x)),
        }
    }
}
