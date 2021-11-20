// struct Sequence{
//     list:
// }
//
// impl Cards {
//     fn init()->Cards{
//         let mut list = ['1','2','3','4','5','6','7','8','9','X','J','Q', 'K'];
//     }
// }

pub fn calculate() {
    let mut list = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'X', 'J', 'Q', 'K'];
    let mut sequence: Vec<char> = Vec::with_capacity(list.len());
    for _ in 0..sequence.capacity() {
        sequence.push('_');
    }
    let mut c = list.len();
    let mut put_index = 0usize;
    let mut i = 0;
    loop {
        let mut c = 0;
        for ii in 0..put_index{

        }

        let c = *unsafe { sequence.get_unchecked(i) };
        if c != ' ' {
            sequence[i] = list[put_index];
            put_index += 1;
        }
        i += 1;
        if i >= sequence.len() {
            i = 0;
        }
        if put_index >= sequence.len() {
            break;
        }
    }
    println!("{:?}, len:{}", sequence, sequence.len());
}



