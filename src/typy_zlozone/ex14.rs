/*
Napisz program, który używa HashMap do przechowywania par klucz-wartość (np. nazwa studenta i jego wynik). Wyświetl wszystkie pary.
 */
use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    name: String,
    marks: Vec<u8>
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct PrincipalId {
    id: String
}

pub fn run_ex14() {
    let mut students:HashMap<PrincipalId, Student> = HashMap::new();

    students.insert(
        PrincipalId { id: "test".to_string()},
        Student {
            marks: vec![1,2,3,4],
            name: "Andrii".to_string()
        }
    );

    students.insert(
        PrincipalId {id: "test2".to_string()},
        Student {
            marks: vec![3,3,3,3],
            name: "Romuald".to_string()
        }
    );

    println!("students: {:?}", students);

    for key in students.keys() {
        println!("{:?}", key);
    }

    for value in students.values() {
        println!("{:?}", value);
    }

}