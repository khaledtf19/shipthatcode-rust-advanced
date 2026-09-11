macro_rules! sum {
    ($($x:expr),*) => {
        {
            let mut sum = 0;
            $(sum+=$x;)*
            sum
        }
    };
}

fn main() {
    let sum = sum!(1, 2, 3, 4, 5);
    println!("{}", sum);
}
