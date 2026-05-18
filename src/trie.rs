pub(crate) struct Trie<T> {
    children: Vec<rustc_hash::FxHashMap<char, usize>>,
    values: Vec<Vec<T>>,
}

impl<T> Trie<T> {
    pub(crate) fn with_capacity(n: usize) -> Self {
        let mut children = Vec::with_capacity(n + 1);
        children.push(Default::default());
        let mut values = Vec::with_capacity(n + 1);
        values.push(vec![]);
        Self { children, values }
    }

    pub(crate) fn insert(&mut self, k: &str, v: T) {
        let mut i = 0;
        for c in k.chars() {
            let j = self.children.len();
            i = *self.children[i].entry(c).or_insert(j);
            if i == j {
                self.children.push(Default::default());
                self.values.push(vec![]);
            }
        }
        self.values[i].push(v);
    }

    pub(crate) fn for_each_vacant(&self, threshold: usize, f: impl FnMut(&str)) {
        todo!()
    }
}
