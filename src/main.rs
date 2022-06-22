macro_rules! printfor {
    () => {};
    ($do:expr,$val:expr,$fmt:expr) => {
        match $do {
            true => {
                for _ in 0..=$val {
                    print!(concat!($fmt, "\n"))
                }
            }
            false => {
                for _ in 0..=$val {
                    print!($fmt);
                }
            }
        }
    };
}

fn main() {
    printfor!(true, 100, "人生終わり");
    printfor!(false, 100, "人生終わり");
}
