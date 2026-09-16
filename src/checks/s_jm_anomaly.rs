use crate::{entry::Single, keytao::*};
use ahash::AHashMap;

struct Node {
    start: usize,
    end: usize,
    non_full: bool,
    bad_all: bool,
    bad_rest: bool,
}

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut cnt = AHashMap::with_capacity(singles.len());
    for e in singles {
        *cnt.entry(&*e.code).or_default() += 1;
    }

    let mut order = Vec::from_iter(0..singles.len());
    order.sort_unstable_by_key(|&i| (singles[i].text, &singles[i].code, i));

    let mut bad = vec![false; singles.len()];
    let mut nodes = vec![];

    for g in order.chunk_by(|&a, &b| singles[a].text == singles[b].text) {
        process(g, singles, &cnt, &mut bad, &mut nodes);
    }

    order
        .into_iter()
        .filter_map(|i| bad[i].then_some(&singles[i]))
        .collect()
}

fn process(
    group: &[usize],
    singles: &[Single],
    cnt: &AHashMap<&str, usize>,
    bad: &mut [bool],
    nodes: &mut Vec<Node>,
) {
    nodes.clear();

    let mut i = 0;
    while i < group.len() {
        let mut j = i + 1;
        while j < group.len() && singles[group[j]].code == singles[group[i]].code {
            j += 1;
        }
        nodes.push(Node {
            start: i,
            end: j,
            non_full: false,
            bad_all: false,
            bad_rest: false,
        });
        i = j;
    }

    for ni in 0..nodes.len() {
        let code = &singles[group[nodes[ni].start]].code;
        for n in 1..code.len() {
            if let Some(pi) = find(singles, group, nodes, &code[..n]) {
                nodes[pi].non_full = true;
            }
        }
    }

    for fi in 0..nodes.len() {
        if nodes[fi].non_full {
            continue;
        }

        let full_cnt = nodes[fi].end - nodes[fi].start;
        if full_cnt > 1 {
            nodes[fi].bad_rest = true;
        }

        let full = &singles[group[nodes[fi].start]].code;
        let mut pref = [0usize; MAX_CODE_N];
        let mut pref_n = 0;
        let mut ttl = full_cnt - 1;

        for n in 1..full.len() {
            if let Some(pi) = find(singles, group, nodes, &full[..n]) {
                pref[pref_n] = pi;
                pref_n += 1;
                ttl += nodes[pi].end - nodes[pi].start;
            }
        }

        if ttl > 1 {
            for &pi in &pref[..pref_n] {
                nodes[pi].bad_all = true;
            }
        } else if ttl == 1 && full_cnt == 1 && pref_n == 1 {
            let pi = pref[0];
            if !nodes[pi].bad_all {
                let code = &*singles[group[nodes[pi].start]].code;
                if cnt[code] > 1 || (1..code.len()).any(|n| !cnt.contains_key(&code[..n])) {
                    nodes[pi].bad_all = true;
                }
            }
        }
    }

    for node in nodes.iter() {
        if node.bad_all {
            for &i in &group[node.start..node.end] {
                bad[i] = true;
            }
        } else if node.bad_rest {
            for &i in &group[node.start + 1..node.end] {
                bad[i] = true;
            }
        }
    }
}

fn find(singles: &[Single], group: &[usize], nodes: &[Node], code: &str) -> Option<usize> {
    nodes
        .binary_search_by(|n| (&*singles[group[n.start]].code).cmp(code))
        .ok()
}
