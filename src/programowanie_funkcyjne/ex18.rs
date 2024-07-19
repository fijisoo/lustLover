/*
Napisz program, który tworzy wektor liczb całkowitych i używa iteratora do wyświetlenia tylko parzystych liczb.
 */

pub fn run_ex18(){
    let mut vector: Vec<u8> = vec![];

    for index in 0..20{
        vector.push(index)
    }

    vector.push(94);

    let result: Vec<u8> = vector.iter().filter(|&&val| {
        if(val % 2 == 0){
            true
        }else{
            false
        }
    }).copied().collect();


    println!("test: {:?}", result)
}