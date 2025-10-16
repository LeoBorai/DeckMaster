use std::ops::Deref;

#[derive(Debug, Default, Clone)]
pub struct QuerySetMeta {
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u32,
}

/// Represents the result from a database query along with metadata
#[derive(Debug, Default, Clone)]
pub struct QuerySet<T: Clone> {
    records: Vec<T>,
    count: u64,
    metadata: QuerySetMeta,
}

impl<T: Clone> Deref for QuerySet<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

impl<T: Clone, E: Clone> QuerySet<Result<T, E>> {
    pub fn transmute(self) -> Result<QuerySet<T>, E> {
        let records = self.records.into_iter().collect::<Result<Vec<T>, E>>()?;
        Ok(QuerySet {
            records,
            count: self.count,
            metadata: self.metadata,
        })
    }
}

impl<T: Clone> QuerySet<T> {
    /// Creates a new instance of `QuerySet` with it's records and `count`.
    ///
    /// The `count` value is used to determine the amount of records found
    /// in the query, that may or may not be the total amount of records in
    /// this `QuerySet` instance.
    pub fn new(records: Vec<T>, metadata: QuerySetMeta) -> Self {
        Self {
            count: records.len() as u64,
            records,
            metadata,
        }
    }

    /// Subset of records matched by the database query and also included as
    /// part of the pagination process
    #[inline]
    pub fn records(&self) -> Vec<T> {
        self.records.to_owned()
    }

    /// Total amount of records in the database matched by the query
    #[inline]
    pub fn count(&self) -> u64 {
        self.count.to_owned()
    }

    /// Current page of the `QuerySet`
    #[inline]
    pub fn page(&self) -> u64 {
        self.metadata.page
    }

    /// Amount of records per page
    #[inline]
    pub fn per_page(&self) -> u64 {
        self.metadata.per_page
    }

    /// Total amount of pages available
    #[inline]
    pub fn total_pages(&self) -> u32 {
        self.metadata.total_pages
    }

    /// Creates an empty `QuerySet`. This is useful for results where `T`
    /// doesn't implement `Default`.
    pub fn empty() -> Self {
        Self {
            records: Vec::new(),
            count: 0,
            metadata: QuerySetMeta::default(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Maps records stored internally
    pub fn inner_map<B, F>(self, f: F) -> QuerySet<B>
    where
        B: Clone,
        F: FnMut(T) -> B,
    {
        QuerySet {
            records: self.records.into_iter().map(f).collect::<Vec<B>>(),
            count: self.count,
            metadata: self.metadata,
        }
    }

    /// Maps records stored internally
    pub fn inner_map_while<B, F>(self, f: F) -> QuerySet<B>
    where
        B: Clone,
        F: FnMut(T) -> Option<B>,
    {
        QuerySet {
            records: self.records.into_iter().map_while(f).collect::<Vec<B>>(),
            count: self.count,
            metadata: self.metadata,
        }
    }
}
