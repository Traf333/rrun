fn main() {
   
    let command = std::env::args().nth(1).expect("Command is not given");
    let number = std::env::args().nth(2).expect("Number is not given");
     println!("Hello {}, world choose {}!", command, number);
}
