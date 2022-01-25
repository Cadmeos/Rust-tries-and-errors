fn main () {   
    //LIFO data structure test in rust. 
    //Last-in First-out.

    let mut stack = Vec::new();

    stack.push("Doom");
    stack.push("FFVII");
    stack.push("Enlisted");
    stack.push("Garry's Mod");
    stack.push("CS:GO");

    let best_game = stack.pop();
    //.pop() pops a Type out of the stack
    // that can be used with a variable

    println!("{:#?}", stack); // The entire Stack
    println!("{:?}", best_game); // CS:GO
    println!("{:?}", stack.capacity()); //8
    println!("{:?}", stack.is_empty()); //false

 }
