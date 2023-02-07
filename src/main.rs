use std::collections::HashMap;

fn main() {
    let empty: HashMap<&str, usize> = hashmap! {};
    let empty_nums = hashmap! { &str: usize; };
    let ascii_table = hashmap! {
        65: "A",
        66: "B",
        67: "C"
    };
    let player_lvls = hashmap! {
        "Bilbo Baggins": 20,
        "Doc": 50,
        "Darth Vader": 250,
    };
    let uname_pwords = hashmap! {
        String: &str;
        { String::from("Bilbo Baggins") }: "r!ng0fP@w3r",
        { "Doc".to_string() }: "<b4ck2TH3future",
        { "Darth Vader".to_owned() }: "Th3nuw!|ldi3Br4vErthanm0st"
    };
    
    let a = vec![1, 2, 3];
    let b = vec![3, 1, 4, 1, 5, 9, 2, 6, 8, 5, 3, 5];
    let e = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0, 4, 5];
    let z = vec![0u8];
    let y = vec![3u8, 4, 5];
    let x: Vec<u8> = vec![];
    let vec_to_vecs = hashmap! {
        &Vec<i32>: Vec<u8>;
        &a: z,
        &b: y,
        &e: x
    };
    
    let a = 1isize;
    let b = 2isize;
    let c = 3isize;
    let num_to_map = hashmap! {
        isize: HashMap<&str, Option<char>>;
        a: { 
            hashmap! {
                "a": { Some('0') },
                "b": { None },
            }
        },
        b: {
            hashmap! {
                &str: Option<char>;
                "a": { Some('1') },
                "b": { Some('0') },
                "c": None,
            }
        },
        c: {
            hashmap! {
                "a": { Some('2') },
                "b": { Some('1') },
                "c": { Some('0') },
                "d": None,
            }
        },
    };
    
    println!("empty: {:?}", &empty);
    println!("empty_nums: {:?}", &empty_nums);
    println!("ascii_table: {:#?}", &ascii_table);
    println!("player_lvls: {:#?}", &player_lvls);
    println!("uname_pwords: {:#?}", &uname_pwords);
    println!("vec_to_vecs: {:#?}", &vec_to_vecs);
    println!("num_to_map: {:#?}", &num_to_map);
    
    // It will give a bunch of warning saying that there are unnecessary braces, but I don't know how to fix that.
}
