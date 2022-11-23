use std::fmt::Debug;

pub struct MinHeapEntry<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> MinHeapEntry<K, V> {
    pub fn new(k: K, v: V) -> Self {
        MinHeapEntry { key: k, value: v }
    }
}

impl<K: Ord, V> PartialEq for MinHeapEntry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for MinHeapEntry<K, V> {}

impl<K: Ord, V> PartialOrd for MinHeapEntry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for MinHeapEntry<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reversed so a MaxHeap becomes a MinHeap
        self.key.cmp(&other.key).reverse()
    }
}

impl<K: Debug, V: Debug> Debug for MinHeapEntry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinHeapEntry {{ {:?}, {:?} }}", self.key, self.value)
    }
}
