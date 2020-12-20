pub enum Operation<K, V> {
    Insert(K, V),
    Get(K),
    Delete(K),
}
