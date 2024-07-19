/*
Napisz program, który tworzy wektor liczb całkowitych, dodaje do niego kilka wartości, a następnie wyświetla wszystkie elementy.
 */
pub fn run_ex10(){
    let mut vector: Vec<u8> = vec![1,2,3,4];

    for index in 0..3 {
        vector.push(index);
    }

    vector.pop();

    println!("data: {:?}", vector)
}