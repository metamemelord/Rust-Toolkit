use std::io::stdin;

macro_rules! r {
    ($var: ident : $type:ty) => {
        let mut buf = String::new();
        let _r = stdin().read_line(&mut buf).expect("Error reading line");
        let $var = buf.trim().trim_end().parse::<$type>().unwrap();
    };
}

macro_rules! rv {
    ($var: ident : $type:ty) => {
        let mut buf = String::new();
        let _r = stdin().read_line(&mut buf).expect("Error reading line");
        let $var: Vec<$type> = buf
            .trim_end()
            .split_ascii_whitespace()
            .map(|x| x.parse::<$type>().unwrap())
            .collect();
    };
}

fn read<T: std::str::FromStr>(t: &mut T) {
    let mut buf = String::new();
    let _r = stdin().read_line(&mut buf).expect("Error reading line");
    match buf.trim().trim_end().parse::<T>() {
        Ok(val) => *t = val,
        Err(_) => println!("Failed to parse"),
    }
}

fn read_vec<T: std::str::FromStr>(t: &mut Vec<T>) {
    let mut buf = String::new();
    let _r = stdin().read_line(&mut buf).expect("Error reading line");
    *t = buf
        .trim_end()
        .split_ascii_whitespace()
        .map(|x| match x.trim().parse::<T>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .map(|x| x.unwrap())
        .collect();
}

fn main() {
    r!(_x: i32);
    rv!(x: i32);
}
