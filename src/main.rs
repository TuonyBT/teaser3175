use std::collections::BTreeSet;

//  Let there be s segments, of which w are winners. These are the same for each player.
//  For a given randomisation:
//  let x winning segments have at least another one immediately clockwise,
//  let y winning segments have at least another two immediately clockwise (therefore y < x),
//  let z winning segments have at least another three immediately clockwise (therefore z < y).

//  So for a given randomisation:
//  P(win | M) = x/w
//  P(win | MM) = x/w * y/x
//  P(win | M | M) = y/x
//  P(win | MMM) = x/w * y/x * z/y
//  P(win | M | MM) = z/y

//  In all cases, P(win | S) = w / s, with no condition on previous moves

fn main() {

    //  analysing out z = w^3 / s ^2 for MMMS, list all [w, s] pairs that satisfy this requirement,
    //  such that s < 100 and 2w < s
    let mut ws_mmms = BTreeSet::<[u32; 2]>::new();

    //  We are told that P(win | MMMS) = P(win | SSS)
    //  i.e. x/w * y/x * z/y * w/s = (w/s)^3
    //  so z = w^3 / s^2 >= 1

    //  Now let w = z*wp^2 where z = product(non-square factors) and wp = product(square factors)
    //  ...considering prime factors of w: "square" have even exponents and "non-square" have odd exponents
    //  e.g. if w = 18, factors are 2e1 * 3e2, so z = 2 and wp = 3

    //  so w^3 = (z.wp^2)^3 = z^2 * (wp^3)^2 = z.s^2
    //  therefore s = z.wp^3
    //  but s > 2*w, so z.wp^3 > 2*z*wp^2, implying wp > 2 (lower limit for wp)

    //  also s < 100 means that z.wp^3 < 100 so wp < cube root(100 / z) (upper limit for wp in terms of z)
    //  wp >= 3 implies that cube root(100 / z) >= 3 so z < 4 (upper limit for z)

    //  thus:
    for z in 1..4 {
        for wp in 3..(100.0 / z as f64).powf(1.0 / 3.0).ceil() as u32 {
            let w = z * wp * wp;
            let s_sq = w.pow(3) / z;
            let s = (s_sq as f64).sqrt() as u32;
            ws_mmms.insert([w, s]);
        }
    }
    //  results in all pairs of [s, w] that are consistent with the MMMS result


    //  We are also told that P(win | MMSM) = P(win | SSS)
    //  i.e. x/w * y/x * w/s * x/w = (w/s)^3 = x.y/(w.s)
    //  so x.y = w^4 / s^2 = (w^2 / s)^2
    //  we use this to filter our list of [s, w] pairs

    for [w, s] in ws_mmms {
        let xy = w.pow(4)/ s.pow(2);

        // find all valid values of x and y (both integers, x > y) that give this product
        let xy_sqrt = w.pow(2) / s;
        for x in (xy_sqrt + 1)..w { // x > y and x < w

            if xy % x != 0 {continue}   // y must be an integer
            let y = xy / x;
    
            let x_w = x as f64 / w as f64;
            let y_x = y as f64 / x as f64;
            let w_s = w as f64 / s as f64;

            if x_w > w_s && y_x > w_s {// choice of M in 1st and 2nd turns of MMSM implies this
                println!("There were {} sectors, of which {} were winners.", s, w);
            }
        }
    }

}