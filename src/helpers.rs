pub fn combinations(state: &Vec<u64>, n: u32) -> Vec<Vec<u64>> {
    if n == 0 || n > state.len() as u32 {
        return vec![];
    }

    if n == state.len() as u32 {
        return vec![state.to_vec()];
    }

    if n == 1 {
        return state.iter().map(|x| vec![*x]).collect::<Vec<Vec<u64>>>();
    }

    let mut res = vec![];

    for i in 0..=state.len() as u32 - n {
        let current = state[i as usize];
        let remaining = &state[i as usize + 1..].to_vec();

        for mut sub in combinations(remaining, n - 1) {
            let mut new = vec![current];
            new.append(&mut sub);
            res.push(new);
        }
    }

    res
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    let i = a.trailing_zeros();
    let j = b.trailing_zeros();
    let k:u32;
    if i<j{k=i}
    else {k=j}
    a >>= i;
    b >>= j;
    loop {
        if a > b {
            a^=b;
            b^=a
            ;a^=b;
        }
        b -= a;
        if b == 0 {
            return a << k;
        }
        b >>= b.trailing_zeros();
    }
}

pub fn gcd_vec(a: Vec<u64>) -> u64 {
    let mut res = a[0];
    for i in 1..a.len() {
        let val = a[i];
        res = gcd(val, res);
    }
    res
}

pub fn factorize(a: u64) -> u64 {
    todo!("get the prime factorization of a");
}
