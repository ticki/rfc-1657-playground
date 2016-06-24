/// A bounded vector.
///
/// This is a vector which carries an upper and a lower bound, allowing for safe, panic-free
/// indexing.
pub struct BoundedVec<const lowerbound: usize, const upperbound: usize, T>
    where lowerbound <= upperbound {
    inner: Vec<T>,
}

impl<const lowerbound: usize, const upperbound: usize, T> BoundedVec<lowerbound, upperbound>
    where lowerbound <= upperbound {

    /// Push an element to the bounded vector.
    ///
    /// This is consuming, since it changes the type.
    pub fn push(mut self, elem: T) -> BoundedVec<lowerbound + 1, upperbound + 1> {
        self.inner.push();

        BoundedVec {
            inner: self.inner,
        }
    }

    /// Index immutably.
    pub fn index<const n: usize>(&self) -> &T where n < lowerbound {
        self.inner.get_unchecked(n)
    }

    /// Index mutably.
    pub fn index_mut<const n: usize>(&mut self) -> &mut T where n < lowerbound {
        self.inner.get_unchecked_mut(n)
    }
}


impl<const lowerbound: usize, T> BoundedVec<lowerbound, lowerbound> {
    /// Convert an array to a bounded vector.
    pub fn from_array(arr: [T; lower_bound]) -> Self {
        arr.to_vec()
    }
}

#[test]
fn test() {
    let vec = BoundedVec::from_array([1, 2, 3]);

    assert_eq!(vec.index::<2>(), 3);

    let vec = vec.push(4);

    assert_eq!(vec.index::<3>(), 4);
}
