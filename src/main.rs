use std::collections::BTreeMap;

fn main() {

    let mut sw_pairs = BTreeMap::<[u32; 2], Vec<[u32; 3]>>::new();

    for s in 9_u32..100 {
        for w in 4.. (s / 2 + 1) {
            if w.pow(2) % s != 0 {continue}

            let xy = w.pow(4) / s.pow(2);
            let xy_pf = prime_factor(xy as usize);

            if xy_pf.len() < 2 {continue}
            let z = xy / w;

            for x in xy.pow(1/2).. w {
                if xy % x != 0 || xy / x >= x || xy % w != 0 {continue}
                let y = xy / x;

                let x_w = x as f64 / w as f64;
                let y_x = y as f64 / x as f64;
                let z_y = z as f64 / y as f64;
                let w_s = w as f64 / s as f64;
//                println!(" x/w: {}; y/x: {}; z/y: {}; w/s: {}", x_w, y_x, z_y, w_s);

//                if x_w > w_s && y_x > w_s {
//                    println!(" s: {}; w: {}; x: {}; y: {}; z: {}; z/y > w/s: {}", s, w, x, y, z, z_y > w_s);
//                }

                sw_pairs.entry([s, w]).or_insert(Vec::<[u32; 3]>::new()).push([x, y, z]);
//                println!(" s: {}; w: {}; x: {}; y: {}; z: {}", s, w, x, y, z);
            }

//            println!("{}, {}", s, w);
//            println!("{}, {:?}", xy, xy_pf);
        }
    }

    for &[s, w] in sw_pairs.keys() {
        let z = w.pow(3) / s.pow(2);
        let z_rem = w.pow(3) % s.pow(2);
        println!("s: {}, w: {}, z: {}, z_rem: {}, xyz: {:?}", s, w, z, z_rem, sw_pairs[&[s, w]]);
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
