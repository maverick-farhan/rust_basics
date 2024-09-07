use std::io;

fn main() {
    println!("
_________                                           __
\\_   ___ \\   ____    ____ ___  __  ____ _______ _ /  |_   ____ _______ 
/    \\  \\/  /  _ \\  /    \\  \\/ /_/ __ \\_  __ \\   __\\_/ __ \\_  __ \\n
\\     \\____(  <_> )|   |  \\   / \\  ___/ |  | \\/ |  |  \\  ___/ |  | \\/
 \\______  / \\____/ |___|  / \\_/   \\___  >|__|    |__|   \\___  >|__|
        \\/              \\/            \\/                    \\/");
loop{
let mut celsius = String::new();
print!("|-------------------------------------------------------------|\n");
println!("  Enter temperature in Celsius");
println!("  Type numbers to continue using or \"e\" to quit!");
print!("|-------------------------------------------------------------|\n");
print!("->");
    io::stdin()
    .read_line(&mut celsius)
    .expect("Failed to get temperature: !");
let celsius:f32 = celsius.trim().parse().expect("Error in converting string to real number");
let result = temp_in_cel(celsius);
println!(" Temperature in Fahrenhiet: {result}");
}
}

fn temp_in_cel(cel:f32)->f32{
    let fah:f32 = ((9.0/5.0) * cel)+32.0;
    fah
}
// Code to exit the program
// let mut exit_or_not = String::new();
// println!("Type (y)es to quit and (n)o to continue!");
// io::stdin()
// .read_line(&mut exit_or_not)
// .expect("Wrong command");
// if exit_or_not == "exit" {
//     break;
// }
