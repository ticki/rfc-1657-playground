impl AtomicU8 {
    /// Load this value.
    ///
    /// Compilation will fail if ordering is `Release` or `AcqRel`.
    pub fn load<const ordering: Ordering>(&self) -> u8
        where ordering != Ordering::Release,
              ordering != Ordering::AcqRel {
        // ...
    }
}
