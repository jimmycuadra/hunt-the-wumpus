use stdweb::unstable::TryInto;

pub fn js_rand(bottom: u8, top: u8) -> u8 {
    let rand = js! {
        return Math.random();
    };
    let base: f64 = rand.try_into().unwrap();

    (base * top as f64).floor() as u8 + bottom
}

pub fn gen_range_avoiding(bottom: u8, top: u8, avoid: Vec<u8>) -> u8 {
    let mut ret = avoid[0];

    while avoid.contains(&ret) {
        ret = js_rand(bottom, top);
    }

    ret
}

pub fn room_exits(id: u8) -> [u8; 3] {
    match id {
        1 => [2, 5, 8],
        2 => [1, 3, 10],
        3 => [2, 4, 12],
        4 => [3, 5, 14],
        5 => [1, 4, 6],
        6 => [5, 7, 15],
        7 => [6, 8, 17],
        8 => [1, 7, 11],
        9 => [10, 12, 19],
        10 => [2, 9, 11],
        11 => [8, 10, 20],
        12 => [3, 9, 13],
        13 => [12, 14, 18],
        14 => [4, 13, 15],
        15 => [6, 14, 16],
        16 => [15, 17, 18],
        17 => [7, 16, 20],
        18 => [13, 16, 19],
        19 => [9, 18, 20],
        20 => [11, 17, 19],
        other => panic!("asked for exits for invalid room {}", other),
    }
}
