//! CreateResult values.
//!
//! Type [`CreateResult`] represents an optional value: every [`CreateResult`]
//! is either [`Created`] or [`Duplicate`], and contains a value.

/// The `CreateResult` type. See [the module level documentation](self) for
/// more.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum CreateResult<T> {
    Created(T),
    Duplicate(T),
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl<T> CreateResult<T> {
    /// Returns the contained [`Created`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is dedicated for integration
    /// tests only.
    #[cfg(feature = "integration-tests")]
    #[inline]
    #[allow(dead_code)]
    pub fn unwrap_created(self) -> T {
        match self {
            CreateResult::Created(item) => item,
            CreateResult::Duplicate(_) => panic!("called `CreateResult::unwrap_created()` on an `Duplicate` value"),
        }
    }

    /// Returns the contained [`Duplicate`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is dedicated for integration
    /// tests only.
    #[cfg(feature = "integration-tests")]
    #[inline]
    #[allow(dead_code)]
    pub fn unwrap_duplicated(self) -> T {
        match self {
            CreateResult::Created(_) => panic!("called `CreateResult::unwrap_duplicated()` on an `Created` value"),
            CreateResult::Duplicate(item) => item,
        }
    }

    /// Returns the contained [`Created`] or [`Duplicate`] value, consuming the
    /// `self` value. # Examples
    ///
    /// ```
    /// use auth_config::domain::entity::CreateResult;
    ///
    /// let x = CreateResult::Created("created instance");
    /// assert_eq!(x.unwrap(), "created instance");
    ///
    /// let x = CreateResult::Duplicate("duplicate instance");
    /// assert_eq!(x.unwrap(), "duplicate instance");
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub fn unwrap(self) -> T {
        match self {
            CreateResult::Created(item) => item,
            CreateResult::Duplicate(item) => item,
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the result is a [`CreateResult::Created`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use auth_config::domain::entity::CreateResult;
    ///
    /// let x = CreateResult::Created(2);
    /// assert_eq!(x.is_created(), true);
    ///
    /// let x = CreateResult::Duplicate(5);
    /// assert_eq!(x.is_created(), false);
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub const fn is_created(&self) -> bool {
        matches!(*self, CreateResult::Created(_))
    }

    /// Returns `true` if the result is a [`CreateResult::Duplicate`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use auth_config::domain::entity::CreateResult;
    ///
    /// let x = CreateResult::Duplicate(2);
    /// assert_eq!(x.is_duplicate(), true);
    ///
    /// let x = CreateResult::Created(5);
    /// assert_eq!(x.is_duplicate(), false);
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub const fn is_duplicate(&self) -> bool {
        matches!(*self, CreateResult::Duplicate(_))
    }

    /// Maps an `CreateResult<T>` to `CreateResult<U>` by applying a function to
    /// a contained value.
    ///
    /// # Examples
    ///
    /// Converts an <code>CreateResult<[String]></code> into an
    /// <code>CreateResult<[usize]></code>, consuming the original:
    ///
    /// [String]: ../../std/string/struct.String.html "String"
    /// ```
    /// use auth_config::domain::entity::CreateResult;
    ///
    /// let maybe_duplicate_string = CreateResult::Duplicate(String::from("Hello, World!"));
    ///
    /// // `CreateResult::map` takes self *by value*, consuming `maybe_duplicate_string`
    /// let maybe_duplicate_string_len = maybe_duplicate_string.map(|s| s.len());
    ///
    /// assert_eq!(maybe_duplicate_string_len, CreateResult::Duplicate(13));
    #[inline]
    #[allow(dead_code)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> CreateResult<U> {
        match self {
            CreateResult::Created(x) => CreateResult::Created(f(x)),
            CreateResult::Duplicate(x) => CreateResult::Duplicate(f(x)),
        }
    }
}
