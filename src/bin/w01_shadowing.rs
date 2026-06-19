fn main() {
    // mut를 붙여서 가변으로 변경
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // 쉐도잉
    let s = 5;

    let s = s + 1;

    {
        let s = s * 2;
        println!("The value of x in the inner scope is: {s}");
    }

    println!("The value of x is: {s}");
}