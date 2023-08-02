use std::collections::BTreeMap;

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

//  We are told that P(win | MMSM) = P(win | SSS)
//  i.e. x/w * y/x * w/s * x/w = (w/s)^3 = x.y/(w.s)
//  so x.y = w^4 / s^2 = (w^2 / s)^2

//  We are also told that P(win | MMMS) = P(win | SSS)
//  i.e. x/w * y/x * z/y * w/s = (w/s)^3
//  so z = w^3 / s^2 >= 1
//  i.e. in all cases w^3 / s^2 is an integer (z), which requires that z * w^3 is a square
//  so either w is a square (and therefore so is z)
//  or w has at least one prime factor with an odd number of powers, 
//  and the lowest possible value for z is the product of those primes

//  if w^3 / s^2 >= 1 and s > 2.w then w^3 >= 4 * w^2, so w >= 4

fn main() {

    // we'll collect all pairs (w, s) that meet the criteria, even if there is only one
    let mut sw_pairs = BTreeMap::<[u32; 2], Vec<[u32; 2]>>::new();

    // out of interest to see how many of the possible ~2500 pairs we actually need to test
    let mut w_counter = 0;
    let mut ws_counter = 0;

    for w in 4_u32..50 {

        // product of prime factors of w with an odd number of powers to give lowest feasible value for z in MMMS scenario
        let z_facs = prime_factor(w as usize).into_iter()
                                    .filter(|[_fac, pwr]| pwr%2 == 1)
                                    .map(|[fac, _pwr]| fac)
                                    .product::<usize>() as u32;

        // largest possible value of s uses all factors of w with even powers                            
        let s_max = ((w.pow(3) / z_facs) as f64).sqrt().floor() as u32;
        if s_max <= w * 2 {continue}

        w_counter += 1;

        // s must be greater than twice w and we are told it is less than 100; we also know it cannot be greater than w^(3/2)
        for s in (w * 2 + 1)..(s_max + 1).min(100) {
            ws_counter += 1;
            if w.pow(3) % s.pow(2) != 0 {continue} // because z for MMMS is an integer

            let xy = w.pow(4) / s.pow(2); // from P(win | MMSM) = P(win | SSS)

            // find all valid values of x and y that give this product
            let xy_sqrt = w.pow(2) / s;
            for x in (xy_sqrt + 1)..w { // x > y and x < w

                if xy % x != 0 {continue}   // y must be an integer
                let y = xy / x;
    
                let x_w = x as f64 / w as f64;
                let y_x = y as f64 / x as f64;
                let w_s = w as f64 / s as f64;

                if x_w > w_s && y_x > w_s {// choice of M in 1st and 2nd turns of MMSM implies this
                    sw_pairs.entry([s, w]).or_insert(Vec::<[u32; 2]>::new()).push([x, y]);
                    // as long as z is less than y for MMSM, the actual value of z places no constraints on w and s
                }
            }
        }
    }

    // Finally check that this set of s and w, plus the corresponding x and y values, are consistent with MMMS
    for &[s, w] in sw_pairs.keys() {
        let z = w.pow(3) / s.pow(2);

        for [x, y] in &sw_pairs[&[s, w]] {
            if y > &z {
                println!("Allowed values of s and w are:");
                println!(" s: {}, w: {}", s, w);        
                println!("and these imply values for x and y of:");
                println!(" x: {}, y: {}", x, y);        
                println!("and z for the MMMS sequence: {}", z);
            }
        }

        println!();
        println!("Iterations of w: {} and w_s: {}", w_counter, ws_counter);
    }

}

//  Prime factor finder ported from Jim Randell's Enigma Python library
//  Wheel factorisation using wheels of circumference 30
pub fn prime_factor(m: usize) -> Vec<[usize; 2]> {
    let mut factors: Vec<[usize; 2]> = Vec::new();
    if m > 1 {
        let mut n = m;
        let mut i = 2;
        let ds = [1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6];
        let js = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 3];
        let mut j = 0;
        while i*i <= n {
            let mut e = 0;
            loop {
                let (d, r) = (&n/&i, &n%&i);

                if r > 0 {
                    break;
                }
            e += 1;
            n = d;
            }
            if e > 0 {
                factors.push([i, e]);
            }
            i += ds[j];
            j = js[j];
        }
        if n > 1 {
            factors.push([n, 1]);
        }
    }
    factors
}
