struct Node<T> {
    next: rustc_hash::FxHashMap<char, usize>,
    values: Vec<T>,
}

impl<T> Node<T> {
    fn new() -> Self {
        Self {
            next: Default::default(),
            values: vec![],
        }
    }
}

pub(crate) struct Trie<T> {
    pool: Vec<Node<T>>,
}

impl<T> Trie<T> {
    pub(crate) fn with_capacity(n: usize) -> Self {
        let mut pool = Vec::with_capacity(n);
        pool.push(Node::new());
        Self { pool }
    }

    pub(crate) fn for_each_by_key<F: FnMut(&T)>(&self, k: &str, exact: bool, mut f: F) {
        let mut i = 0;
        for c in k.chars() {
            let Some(&j) = self.pool[i].next.get(&c) else {
                return;
            };
            i = j;
        }
        for v in &self.pool[i].values {
            f(v);
        }
        if exact {
            return;
        }
        let mut indexes = Vec::with_capacity(64);
        indexes.extend(self.pool[i].next.values().copied());
        while let Some(i) = indexes.pop() {
            for v in &self.pool[i].values {
                f(v);
            }
            indexes.extend(self.pool[i].next.values().copied());
        }
    }

    pub(crate) fn insert(&mut self, k: &str, v: T) {
        let mut i = 0;
        for c in k.chars() {
            let j = self.pool.len();
            i = *self.pool[i].next.entry(c).or_insert(j);
            if i == j {
                self.pool.push(Node::new());
            }
        }
        self.pool[i].values.push(v);
    }
}
