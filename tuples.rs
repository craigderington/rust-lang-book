// tuples have a max of 12 elements

pub fn run () {
    let person: (&str, &str, i8) = ("James", "Mass", 37);
    println!("{} is from {} as is {} yeard old", person.0, person.1, person.2);
}