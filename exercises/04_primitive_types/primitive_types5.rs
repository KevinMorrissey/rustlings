fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;
    //further look at tupling shows I don't have to structure it by declaration and then assigning, can just be done in the tuple destructure.
    //let name = x;
   // let age = y;
    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    println!("{name} is {age} years old");
}