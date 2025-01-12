pub fn loops() {
    let nums: [i8; 4] = [1, 2, 3, 4];
    let multiples: [i8; 4] = [2, 4, 6, 8];
    let mut index_outer = 0;
    'iteratable: loop {
        if index_outer == nums.len() {
            break;
        }
        println!("multiples of {0}", nums[index_outer]);
        let mut index_inner = 0;
        loop {
            if index_inner == multiples.len() {
                break;
            }

            if nums[index_outer] == 4 {
                println!("can not process 4");
                break 'iteratable;
            }

            println!("{}", nums[index_outer] * multiples[index_inner]);
            index_inner = index_inner + 1;
        }
        index_outer += 1;
    }
}
