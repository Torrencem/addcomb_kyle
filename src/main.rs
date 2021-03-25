
use addcomb_comp::setlike::SetLike;
use addcomb_comp::setlike::Group;
use addcomb_comp::exactset;
use addcomb_comp::comb::gcd;

/// Compute whether there is a set A of size two such that [ia, ib]A = G
/// If there is such a set, this function returns one of these sets. Otherwise, it returns None.
fn do_two_elements_span(k: u32, s: u32) -> Option<Vec<GElem>> {
    let g = Rc::new(vec![2, 2 * k]);
    let g_size = g.gsize();

    if 4*k > 2*s*s + 2*s + 1 {
        return None;
    }

    for a in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
        let p = &a[0];
        let q = &a[1];
        let c = p.0[0];
        let x = p.0[1];
        let d = q.0[0];
        let y = q.0[1];

        // Optimization 1
        if c == 0 && d == 0 {
            continue;
        }

        // Optimization 2
        if gcd(2*k, gcd(x, y)) != 1 {
            continue;
        }

        // Optimization 3
        // Compute if x and y are not in [k / s, -k / s]
        let lb = k / s;
        let ub = 2*k - (k / s);
        if (x < lb || x > ub) && (y < lb || y > ub) {
            continue;
        }

        let size = exactset::hfold_interval_signed_sumset(&a, (0, s), g.clone()).len() as u32;
        if size == g_size {
            return Some(a);
        }
    }
    None
}

use std::rc::Rc;
use addcomb_comp::exactset::GElem;

fn main() {
    for s in 2..100 {
        println!("s = {}", s);
        for k in 5..((2*s*s + 2*s + 1) / 4) {
            // Compute do_two_elements_span
            let comp_result =
                do_two_elements_span(k, s);

            match comp_result {
                Some(set) => {
                    println!("{}: {:?}", k, set);
                },
                None => {
                    // println!("~{}", k);
                },
            }
        }
    }
}

