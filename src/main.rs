use core::f64;

fn main() {
    use std::io;
    use std::time::Instant;
    let mut pi: f64 = 4.0;
    println!("How many times do you want it to calculate? Every 100.000.000 is about one second of processionf time. Only type even numbers.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_i32: i32 = input.trim().parse::<i32>().expect("REASON");
    let now = Instant::now();
    for i in 0..= input_i32{
        let den: i64 = i as i64 * 2 as i64 + 3 as i64;
        if i % 2 == 0{
            pi -= 4 as f64 / den as f64
            ;
        } else {
            pi += 4 as f64 / den as f64;
        }
    }
    println!("{}", pi.to_string());
    if std::f64::consts::PI < pi {
        println!("{}", std::f64::consts::PI / pi * 100.0);
    }else {
        println!("{}", pi / std::f64::consts::PI * 100.0);
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}