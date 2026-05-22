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

    pub(crate) fn for_each_vacant(&self, threshold: usize, mut f: impl FnMut(&str)) {
        self.dfs(0, 0, &mut String::new(), threshold, &mut f);
    }

    fn dfs(&self, i: usize, d: usize, p: &mut String, th: usize, f: &mut impl FnMut(&str)) -> bool {
        let mut v = !self.values[i].is_empty();
        for (&c, &j) in &self.children[i] {
            p.push(c);
            v |= self.dfs(j, d + 1, p, th, f);
            p.pop();
        }
        if d >= th && self.values[i].is_empty() && v {
            f(p);
        }
        v
    }
}
