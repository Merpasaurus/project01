use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for i in 0usize..10 {
        turtle.forward(30.0);

        // Returns either None, or Some(remainder)
        let rem = i.checked_rem(2);

        if rem == Some(1){
            // Odd
            turtle.right(90.0);
        }else{
            // Even
            turtle.right(-90.0);
        }
    }
}