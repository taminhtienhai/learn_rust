fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut total = 0;
    let pointer = flowerbed.clone();

    if flowerbed.len() == 1 && flowerbed.get(0) == Some(&0) {
        total += 1;
    }

    if flowerbed.get(0) == Some(&0) && flowerbed.get(1) == Some(&0) {
        if let Some(it) = flowerbed.get_mut(0) {
            *it = 1;
            total += 1;
        }
    }

    for (index, item) in pointer.iter().enumerate() {
        if *item == 1 || flowerbed.get(index) == Some(&1) {
            let first = flowerbed.get(index + 1);
            let second = flowerbed.get(index + 2);
            let third = flowerbed.get(index + 3);

            if first.is_none() { break; };
            if second.is_none() { break; };
            if third.is_none() {
                if (first.unwrap() + second.unwrap()) == 0 {
                    total += 1;
                }
                break;
            }



            if (first.unwrap() + second.unwrap() + third.unwrap()) == 0 {
                total += 1;

                if let Some(it) = flowerbed.get_mut(index + 2) {
                    *it = 1;
                }
            }
        }
    }


    total >= n
}


fn main() {
    can_place_flowers(vec![0,0,1,0,1], 1);
}