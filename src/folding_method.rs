pub(crate) fn folding_method() {
    const M: u16 = 1000; // 0~999
    const BLOCK: u16 = 3;

    let keys: u64 = 706463482;
    let key_size: usize = keys.to_string().len();
    let divided_key: Vec<u8> = divide_digitize(keys);

    let block_size: usize = key_size / BLOCK as usize;
    let mut surplus: usize = key_size % BLOCK as usize;

    let mut sum: usize = 0;
    let mut current_digit: usize = 0;
    for i in 0..BLOCK {
        let mut sup_tmp: usize = 0;
        if surplus > 0 {
            surplus -= 1;
            sup_tmp = 1;
        }

        let mut tmp: usize = 0;
        let current_block_size: usize = block_size + sup_tmp;
        let mut w: u64 = 10u64.pow((current_block_size - 1) as u32);
        for j in current_digit..(current_digit + current_block_size) {
            tmp += (w * divided_key[j] as u64) as usize;
            w /= 10;
        }
        sum += tmp;
        println!("block{}: {}, sum: {}", i + 1, tmp, sum);
        current_digit += current_block_size; 
    }

    println!("keys: {}, index: {}", keys, sum % M as usize);
}

fn divide_digitize(n: u64) -> Vec<u8> {
    if n > 0 {
        let mut num = n;
        let mut result = Vec::new();
        while num != 0 {
            result.push((num % 10).try_into().unwrap());
            num /= 10;
        }
        result.reverse();
        result
    } else {
        vec![0]
    }
}
