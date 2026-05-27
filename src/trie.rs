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

    pub(crate) fn for_each_vacant(&self, mut f: impl FnMut(&str)) {
        self.dfs(0, 0, &mut String::new(), &mut f);
    }

    fn dfs(&self, i: usize, len: usize, code: &mut String, f: &mut impl FnMut(&str)) -> u8 {
        let mut v = if self.values[i].is_empty() { 0 } else { 1 };
        for (&c, &j) in &self.children[i] {
            code.push(c);
            let cv = self.dfs(j, len + 1, code, f);
            code.pop();
            if cv > 0 {
                v |= cv;
                if len >= 3 && !matches!(c, 'a' | 'i' | 'o' | 'u' | 'v') {
                    v |= 2;
                }
            }
        }
        if self.values[i].is_empty() && (len >= 4 && v != 0 || len == 3 && v == 1) {
            f(code);
        }
        v
    }
}
