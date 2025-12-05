use std::{
    any::type_name,
    cmp::Ordering,
    convert::{From, Into},
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[cfg(feature = "serde")]
mod serde;

type DefaultIdType = u32;

/// ID bound to an owner type T & backed by a type I
///
/// IDs with owners of different types are not interchangeble.
/// Examples:
///
/// ```
/// use typed_id::Id;
/// let id1 = Id::<String>::new(5);
/// let id2 = 5.into(); // From/Into usable
/// assert_eq!(id1, id2);
/// assert_eq!(id1.to_string(), "5");
/// assert_eq!(format!("{:?}", id1), "Id<owner: alloc::string::String, backing: u32>(5)");
/// ```
///
/// ```compile_fail
/// use typed_id::Id;
/// let id1 = Id::<String>::new(1);
/// let id2 = Id::<&str>::new(1);
/// assert_eq!(id1, id2); // cannot compare
/// ```
///
/// ```compile_fail
/// use typed_id::Id;
/// fn do_thing(id: Id::<String>) {}
/// let id = Id::<&str>::new(1);
/// do_thing(id); // cannot pass argument
/// ```
pub struct Id<T, I = DefaultIdType> {
    id: I,
    t: PhantomData<T>,
}

impl<T, I> Id<T, I> {
    /// # Example
    ///
    /// ```
    /// use typed_id::Id;
    ///
    /// struct MyType { id: Id<MyType> }
    ///
    /// let id = Id::<MyType>::new(1);
    /// ```
    pub fn new(id: I) -> Id<T, I> {
        Id::<T, I> { id, t: PhantomData }
    }

    /// # Examples
    ///
    /// ```
    /// use typed_id::Id;
    /// let id_u32 = Id::<String>::new(2);
    /// assert_eq!(id_u32.backing_type(), "u32");
    /// ```
    ///
    /// ```
    /// use typed_id::Id;
    /// let id_i64 = Id::<String, i64>::new(2);
    /// assert_eq!(id_i64.backing_type(), "i64");
    /// ```
    pub fn backing_type(&self) -> &str {
        type_name::<I>()
    }

    /// # Examples
    ///
    /// ```
    /// use typed_id::Id;
    /// let id_str = Id::<String>::new(2);
    /// assert_eq!(id_str.owner_type(), "alloc::string::String");
    /// ```
    ///
    /// ```
    /// use typed_id::Id;
    /// let id_bool = Id::<bool>::new(2);
    /// assert_eq!(id_bool.owner_type(), "bool");
    /// ```
    pub fn owner_type(&self) -> &str {
        type_name::<T>()
    }

    /// # Examples
    ///
    /// ```
    /// use typed_id::Id;
    /// let id_str = Id::<&str>::new(1);
    /// let id_string = Id::<String>::new(1);
    /// let id_bool = Id::<bool>::new(1);
    ///
    /// assert!(id_str.change_owner_type() == id_bool);
    /// assert!(id_bool.change_owner_type() == id_string);
    /// assert!(id_string.change_owner_type() == id_str);
    /// ```
    pub fn change_owner_type<T2>(self) -> Id<T2, I> {
        Id::<T2, I>::new(self.id)
    }

    /// # Examples
    ///
    /// ```
    /// use typed_id::Id;
    /// let id_u8 = Id::<bool, u8>::new(u8::MAX);
    /// let id_u64 = Id::<bool, u64>::new(u8::MAX.into());
    /// let id_i64 = Id::<bool, i64>::new(u8::MAX.into());
    ///
    /// assert!(id_u8.change_backing_type() == id_u64);
    /// assert!(id_u8.change_backing_type() == id_i64);
    /// ```
    pub fn change_backing_type<I2: From<I>>(self) -> Id<T, I2> {
        Id::<T, I2>::new(Into::<I2>::into(self.id))
    }

    /// # Examples
    ///
    /// ```
    /// use typed_id::Id;
    /// let id_u8 = Id::<bool, u8>::new(5);
    /// let id_i8 = Id::<bool, i8>::new(5);
    ///
    /// assert_eq!(id_u8.try_change_backing_type::<i8>(), Ok(id_i8));
    /// assert_eq!(id_i8.try_change_backing_type::<u8>(), Ok(id_u8));
    /// ```
    ///
    /// ```
    /// use typed_id::Id;
    /// let neg = Id::<bool, i8>::new(-1);
    /// assert!(neg.try_change_backing_type::<u64>().is_err());
    /// ```
    pub fn try_change_backing_type<I2: TryFrom<I>>(
        self,
    ) -> Result<Id<T, I2>, <I as TryInto<I2>>::Error> {
        Ok(Id::<T, I2>::new(self.id.try_into()?))
    }
}

impl<T, I> Id<T, I>
where
    I: Clone,
{
    /// # Examples
    ///
    /// ```
    /// use typed_id::Id;
    /// let id = Id::<String>::new(u32::MAX);
    /// assert_eq!(id.value(), u32::MAX);
    /// ```
    ///
    /// ```
    /// use typed_id::Id;
    /// let id = Id::<String, i8>::new(-1);
    /// assert_eq!(id.value(), -1);
    /// ```
    pub fn value(&self) -> I {
        self.id.clone()
    }
}

pub trait HasId<T = Self, I = DefaultIdType> {
    fn id(&self) -> Id<T, I>;
}

impl<T, I: Default> Default for Id<T, I> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            t: PhantomData,
        }
    }
}

impl<T, I> From<I> for Id<T, I> {
    fn from(value: I) -> Self {
        Self::new(value)
    }
}

impl<T, I: Display> Display for Id<T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl<T, I: Debug> Debug for Id<T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id<owner: {}, backing: {}>({:?})",
            type_name::<T>(),
            type_name::<I>(),
            self.id
        )
    }
}

impl<T, I: Clone> Clone for Id<T, I> {
    fn clone(&self) -> Self {
        Self::new(self.id.clone())
    }
}

impl<T, I: Copy> Copy for Id<T, I> {}

impl<T, I: PartialOrd> PartialOrd for Id<T, I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T, I: Ord> Ord for Id<T, I> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T, I: PartialEq> PartialEq for Id<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, I: Eq> Eq for Id<T, I> {}

impl<T, I: Hash> Hash for Id<T, I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

unsafe impl<T, I: Send> Send for Id<T, I> {}
unsafe impl<T, I: Sync> Sync for Id<T, I> {}
