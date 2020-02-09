mod fs;
use fs::taco;

fn main() {
    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(e) => panic!(e)
    };
    println!("{}", home_dir);
    taco();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}