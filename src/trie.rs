pub(crate) struct Trie<T> {
    children: Vec<rustc_hash::FxHashMap<char, usize>>,
    vals: Vec<Vec<T>>,
}

impl<T> Trie<T> {
    pub(crate) fn with_capacity(n: usize) -> Self {
        let mut children = Vec::with_capacity(n + 1);
        children.push(Default::default());
        let mut vals = Vec::with_capacity(n + 1);
        vals.push(vec![]);
        Self { children, vals }
    }

    pub(crate) fn insert(&mut self, k: &str, v: T) {
        let mut i = 0;
        for c in k.chars() {
            let j = self.children.len();
            i = *self.children[i].entry(c).or_insert(j);
            if i == j {
                self.children.push(Default::default());
                self.vals.push(vec![]);
            }
        }
        self.vals[i].push(v);
    }

    pub(crate) fn vacant_codes(&self) -> Vec<String> {
        let mut codes = Vec::new();
        self.dfs(0, &mut String::new(), &mut codes);
        codes
    }

    fn dfs(&self, i: usize, code: &mut String, codes: &mut Vec<String>) -> bool {
        let mut has_val = !self.vals[i].is_empty();
        for (&c, &j) in &self.children[i] {
            code.push(c);
            let child_has_val = self.dfs(j, code, codes);
            code.pop();
            if matches!(c, 'a' | 'i' | 'o' | 'u' | 'v') && child_has_val {
                has_val = true;
            }
        }
        if code.len() >= 3 && self.vals[i].is_empty() && has_val {
            codes.push(code.clone());
        }
        has_val
    }
}
