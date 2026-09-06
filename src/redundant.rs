use crate::{entry::Phrase, keytao::ym};

pub(crate) fn check(phrases: &[Phrase]) -> Vec<&Phrase> {
    let mut sorted: Vec<_> = phrases.iter().collect();

    sorted.sort_unstable_by(|&a, &b| {
        a.text
            .cmp(&b.text)
            .then_with(|| ym(&a.code).cmp(ym(&b.code)))
            .then_with(|| a.code.len().cmp(&b.code.len()))
    });

    let mut prev = None;
    sorted.retain(|&p| {
        let keep = prev.is_some_and(|q| same_chain(q, p));
        prev = Some(p);
        keep
    });

    sorted
}

fn same_chain(a: &Phrase, b: &Phrase) -> bool {
    a.text == b.text && ym(&a.code) == ym(&b.code)
}
