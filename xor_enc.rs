use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut res = Vec::new();

    println!("Enter Text: ");
    io::stdin().read_line(&mut input1).expect("Failed to read first string");

    println!("Enter XOR Key: ");
    io::stdin().read_line(&mut input2).expect("Failed to read second string");

    for (c1,c2) in input1.trim().chars().zip(input2.trim().chars().cycle()){

        let c1 = c1 as u8;
        let c2 = c2 as u8;

        let xor_result = c1 ^ c2;
        res.push(xor_result);

    }
    println!("XOR Result: ");
    println!("In decimal: ");
    println!("{:?}", res);
    println!("In hexadecimal: ");
    for byte in &res{
        print!("{:02x} ", byte);
    }
    println!();

}