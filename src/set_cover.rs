use std::collections::HashSet;

fn set_union(sets: &Vec<HashSet<u32>>) -> HashSet<u32> {
    let mut union_set: HashSet<u32> = HashSet::new();
    for s in sets {
        union_set.extend(s.iter());
    }
    return union_set;
}

pub fn set_cover_possible(set_list: &Vec<HashSet<u32>>) -> bool {
    // First check that there are at least enough elements to go around
    if set_union(set_list).len() < set_list.len() {
        return false;
    }

    // Now sort lists by length
    let mut mut_set_list = set_list.clone();
    mut_set_list.sort_by(|a: &HashSet<u32>, b: &HashSet<u32>| a.len().cmp(&b.len()));

    // The sanity check above means we've already accounted for the longest list
    mut_set_list.pop();

    // The logic is:
    //  - If there are enough elements to go around, pop the longest element
    //  - If there are still enough elements to go around given the smaller size,
    //    set_cover is still possible
    while mut_set_list.len() > 0 {
        if mut_set_list.len() >= mut_set_list[mut_set_list.len() - 1].len() {
            if set_union(&mut_set_list).len() < mut_set_list.len() {
                return false;
            }
        }
        mut_set_list.pop();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_cover_tests() {
        let passing_test: Vec<HashSet<u32>> = vec![HashSet::from([1])];
        assert_eq!(set_cover_possible(&passing_test), true);

        let passing_test: Vec<HashSet<u32>> =
            vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([3])];
        assert_eq!(set_cover_possible(&passing_test), true);

        let passing_test: Vec<HashSet<u32>> = vec![
            HashSet::from([1, 2, 3]),
            HashSet::from([1, 2, 3]),
            HashSet::from([1, 2, 3]),
        ];
        assert_eq!(set_cover_possible(&passing_test), true);

        let failing_test: Vec<HashSet<u32>> = vec![HashSet::from([1]), HashSet::from([1])];
        assert_eq!(set_cover_possible(&failing_test), false);

        let failing_test: Vec<HashSet<u32>> = vec![
            HashSet::from([1]),
            HashSet::from([1]),
            HashSet::from([2, 3]),
        ];
        assert_eq!(set_cover_possible(&failing_test), false);

        // This set should does not cover (but was a buggy output):
        let failing_test: Vec<HashSet<u32>> = vec![
            HashSet::from([1, 2]),
            HashSet::from([1, 2]),
            HashSet::from([1, 2]),
            HashSet::from([3, 4, 5]),
        ];
        assert_eq!(set_cover_possible(&failing_test), false);
    }
}
