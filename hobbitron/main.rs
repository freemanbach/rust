use std::vec::Vec;

fn test(v: u64){
    // two line init for String
    let mut _tmp = String::new();
    _tmp = v.to_string();

    // Final Ans
    let mut _st = String::new();

    // piles of variables to capture solution
    let mut va:Vec<String> = Vec::new();
    let mut ans:Vec<String> = Vec::new();
    let mut z = 0;

    // new pile of variables to capture solution
    let mut fans:Vec<String> = Vec::new();
    let mut tvec:Vec<String> = Vec::new();
    let mut _fst = String::new();

    for i in _tmp.chars() {
        va.push(i.to_string());
    }
    va.sort();
    for i in va.iter() {
        _st+=i;
    }

    // Solutions to the problem
    for i in 2..11 {
        z = i * v;
        ans.push(z.to_string());
    }

    // added organized item to a new list
    for i in ans {
        for y in i.chars() {
            tvec.push(y.to_string());
        }
        tvec.sort();
        for x in tvec.iter() {
            _fst+=x;
        }
        fans.push(_fst);
        _fst = String::new();
    }

}

fn main() {
    test(1246878);
}
