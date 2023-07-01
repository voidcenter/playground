
use std::assert;



fn array_xor (bits: &[bool]) -> bool {
    // println!("{}", bits.len());
    let mut r = bits[0] ^ bits[1];
    for i in 2..bits.len() {
        // println!("mid  r = {}  bits[i] = {}", r, bits[i]);
        r = r ^ bits[i];
    }
    r
}


fn array_or (bits: &[bool]) -> bool {
    // println!("{}", bits.len());
    let mut r = bits[0] | bits[1];
    for i in 2..bits.len() {
        // println!("mid  r = {}  bits[i] = {}", r, bits[i]);
        r = r | bits[i];
    }
    r
}



pub fn test_bool() {
    const n : usize = 10;

    let func1 = | bits: &[bool] | -> bool {
        let bits_len: usize = bits.len().try_into().unwrap();
        assert!(bits_len % 2 == 0);

        let xor1 = array_xor(&bits[0 .. bits_len/2]);
        let xor2 = array_xor(&bits[bits_len/2 .. bits_len]);

        // println!("{} {}", xor1, xor2);

        return xor1 & xor2;
    };

    let func2 = | bits: &[bool] | -> bool {
        let bits_len: usize = bits.len().try_into().unwrap();
        assert!(bits_len % 2 == 0);
        let bits_len2: usize = bits_len / 2;

        let mut v: Vec<bool> = vec![];

        for i in 0..bits_len2 {
            for j in 0..bits_len2 {
                let _and = bits[i] & bits[bits_len2 + j];
                v.push(_and);
            }
        }        
        // println!("{:?}", v);

        array_xor(v.as_slice())
    };


    let mut v: Vec<bool> = vec![];

    const nbits: usize = n * 2;
    const ttable_size: usize = 2_i32.pow(nbits as u32) as usize;
    for x in 0..ttable_size {
        v.clear();
        let mut y = x;
        for i in 0..nbits {
            v.push( if y%2 == 1 { true } else { false } );
            y = y / 2;
        }
        let r1: bool = func1(&v);
        let r2: bool = func2(&v);
        if r1 != r2 {
            println!("{:?} {} {}", v, func1(&v), func2(&v));
        }        
    }


    // let input: &[bool] = &[true, false, true, false];
    // println!("{}", func1(&input));
    // println!("{}", func2(&input));

}


