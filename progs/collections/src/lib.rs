use std::collections::HashMap;

pub fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    let mut i = 0;
    println!("vector length: {}", v.len());
    v.push(2);
    v.push(4);
    v.push(8);
    v.push(16);
    println!("vector length: {}", v.len());
    while i < v.len() {
        println!("{}: {}", i, v[i]);
        i += 1;
    }
    let x = v.pop(); // pops last element: 16 and returns Some(16)
    println!("Popped {:?}! vector length: {}", x, v.len());
    println!("Original popped value: {}", x.unwrap());
    i = 0;
    while i < v.len() {
        println!("{}: {}", i, v[i]);
        i += 1;
    }

    // second vector: create vector from literal values
    let mut v2 = vec![3, 9, 27];
    v2.push(36);
    println!("vector-2 length: {}", v2.len());
    for x in &v2 {
        println!("{x}");
    }

    // third vector
    let v3 = Vec::from([1, 2, 3, 4, 5]);
    println!("Vector-3 of len {}:", v3.len());
    for n in &v3 {
        println!("{n}");
    }
}

pub fn hash_maps() {
    let mut hm: HashMap<u8, bool> = HashMap::new();
    hm.insert(5, true);
    hm.insert(6, false);

    println!("No. of elements in the map: {}", hm.len());
    println!("Printing keys in the hashmap:");
    for key in hm.keys() {
        println!("{key}");
    }

    println!("Printing values in the hashmap:");
    for val in hm.values() {
        println!("{val}");
    }

    println!("Printing keys:values together:");
    for (key, val) in hm.iter() {
        println!("{key}:{val}");
    }

    // second hashmap
    let mut stud = HashMap::from([
        ("Anirudha", 33),
        ("Chandra", 48),
        ("Dhruv", 12),
    ]);
    println!("Students:");
    for (name, age) in &stud {
        println!("{} is {} years old.", name, age);
    }
    // remove an element from the map
    let val = stud.remove(&"Chandra");
    if val == None {
        println!("Chandra is not present in the map");
    } else {
        println!("Chandra stud is removed. His age is {}", val.unwrap());
    }
}