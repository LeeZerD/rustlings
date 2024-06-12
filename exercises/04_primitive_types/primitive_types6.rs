// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.



#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1 ; //Ici on recupère le 2 ème élément de "numbers" sachant que                   
    //l'élément 1 corresponds à "".0" l'élément 2 correponds ici à "".1" de numbers

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
