fn search(target: i32, pack: &Vec<i32>) -> bool {
    /* Since pack is a reference to Vec<i32>, each num will also be a &i32.
     * And we can't compare a &i32 with i32(target). Thus, we need to
     * dereference the num to load the value of num. */
    for num in pack {
        // syntax for dereferencing the num
        if *num == target {
            return true;
        }
    }
    return false;
}

fn main() {
    let pack: Vec<i32> = vec![34, 12, 89, 78, 45, 90, 47, 63];
    let target1: i32 = 90;
    let target2: i32 = 23;

    if search(target1, &pack) {
        println!("{} found in the {:?}", target1, pack);
    }

    if !search(target2, &pack) {
        println!("{} not found in the {:?}", target2, pack);
    }
}
