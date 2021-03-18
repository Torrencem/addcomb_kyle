
use addcomb_comp::setlike::SetLike;
use addcomb_comp::setlike::Group;

/// Compute whether there is a set A of size two such that [ia, ib]A = G
/// If there is such a set, this function returns one. Otherwise, it returns None.
fn do_two_elements_span<S: SetLike>(n: S::Group, (ia, ib): (u32, u32)) -> Option<S> {
    let g_size = n.gsize();
    for a in S::each_set_exact(n.clone(), 2) {
        let size = a.hfold_interval_signed_sumset((ia, ib), n.clone()).size();
        if size == g_size {
            return Some(a);
        }
    }
    None
}

use std::rc::Rc;
use addcomb_comp::exactset::GElem;

fn main() {
    let k = 5;
    // Our group
    let g = Rc::new(vec![2, 2 * k]);

    // Compute do_two_elements_span
    // Note I'm assuming you're taking the [0, 3] fold signed sumset!
    let comp_result =
        do_two_elements_span::<Vec<GElem>>(g, (0, 3));

    match comp_result {
        Some(set) => {
            println!("Found a set that spans our group: {:?}", set);
        },
        None => {
            println!("No sets of size two span the group!");
        },
    }
}
