// primitive_types1.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)


fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = if is_morning == true {
        false
    }else{
        true
    };    
    if is_evening == true{
        println!("Good evening!");
    };
}
