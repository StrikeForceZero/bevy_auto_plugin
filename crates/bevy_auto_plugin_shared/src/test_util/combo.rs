use internal_test_util::vec_spread;
use std::borrow::Borrow;

/// All non-empty combos choosing at most one element from each group.
pub fn combos_one_per_group_or_skip<T: Clone, G>(groups: &[G]) -> Vec<Vec<T>>
where
    G: Borrow<[T]>, // works for Vec<T>, &[T], arrays, etc.
{
    let mut combos = vec![vec![]];

    for g in groups {
        let g = g.borrow(); // &[T]
        let mut next = Vec::with_capacity(combos.len() * (g.len() + 1));

        // option 1: skip this group
        next.extend(combos.iter().cloned());

        // option 2: take exactly one item from this group
        for item in g.iter() {
            for prefix in &combos {
                let mut new = prefix.clone();
                new.push(item.clone());
                next.push(new);
            }
        }

        combos = next;
    }

    // filter out empty sets
    combos.into_iter().filter(|c| !c.is_empty()).collect()
}

/// All non-empty combos choosing at most one element from each group.
/// Inserts `with` item into each group
pub fn combos_one_per_group_or_skip_with<T: Clone, G>(groups: &[G], with: T) -> Vec<Vec<T>>
where
    G: Borrow<[T]>, // works for Vec<T>, &[T], arrays, etc.
{
    let mut combos = vec![vec![]];

    for g in groups {
        let g = g.borrow(); // &[T]
        let mut next = Vec::with_capacity(combos.len() * (g.len() + 1));

        // option 1: skip this group
        next.extend(combos.iter().cloned());

        // option 2: take exactly one item from this group
        for item in g.iter() {
            for prefix in &combos {
                let mut new = prefix.clone();
                new.push(item.clone());
                next.push(new);
            }
        }

        combos = next;
    }

    // filter out empty sets
    combos
        .into_iter()
        .filter(|c| !c.is_empty())
        .map(|g| vec_spread![with.clone(), ..g,])
        .collect()
}
