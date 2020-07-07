use std::vec::Vec;

fn test(v: u64){
    // two line init for String
    let mut tmp = String::new();
    tmp = v.to_string();

    // Final Ans
    let mut st = String::new();

    // piles of variables to capture solution
    let mut va:Vec<String> = Vec::new();
    let mut ans:Vec<String> = Vec::new();
    let mut z = 0;

    // new pile of variables to capture solution
    let mut fans:Vec<String> = Vec::new();
    let mut tvec:Vec<String> = Vec::new();
    let mut fst = String::new();

    for i in tmp.chars() {
        va.push(i.to_string());
    }
    va.sort();
    for i in va.iter() {
        st+=i;
    }

    // Solutions to the problem
    for i in 2..11 {
        z = i * v;
        ans.push(z.to_string());
    }

    // added organized item to a new list
    for i in ans {
        for y in i.chars() {
            println!("{}", y);
            tvec.push(y.to_string());
        }
        tvec.sort();
        for x in tvec.iter() {
            // this is causing major heartburn
            fst+=x;
        }
        println!();
        fans.push(fst);
        // it is not allowing me to clear the string
        fst = String::from("");
    }

    /*
    println!();
    println!("{}", st);
    for i in tvec.iter() {
        println!("{}", i);
    }
    println!();
    for i in fans.iter() {
        println!("{}", i)
    }
    println!();
    */


    if (v != 0) || (v != 1) {
        for i in fans.iter() {
            if  st.eq(i)  {
                println!("Yes we match.");
            }
            else {
                println!("No we dont match.");
            }
        }
    }
}

fn main() {
    test(1246878);
}
