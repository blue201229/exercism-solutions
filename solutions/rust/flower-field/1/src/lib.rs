
pub fn is_flower(garden: &[&str], m:i16, n:i16)->bool{
    if m<0 || n<0{
        return false
    }
    if (n as usize) >= garden.len(){
        return false
    }
    if (m as usize) >= garden[n as usize].len(){
        return false
    }
    if garden[n as usize].as_bytes()[m as usize] == b'*'{
        return true
    }
    false
}
pub fn annotate(garden: &[&str]) -> Vec<String> {

    if garden.len() == 0{
        return vec![]
    }

    let mut res:Vec<String> = Vec::new();
    for n in 0..garden.len(){
        let mut row = String::new();
        for m in 0..garden[n].len(){
            if is_flower(garden, m as i16, n as i16){
                row.push('*');
            }else{
                let mut count = 0;
                for i in -1..=1{
                    for j in -1..=1{
                        if i==0 && j==0{
                            continue
                        }
                        if is_flower(garden, (m as i16)+j, (n as i16)+i){
                            count += 1;
                        }
                    }
                }
                if count == 0{
                    row.push(' ');
                }else{
                    row.push( char::from_digit(count, 10).unwrap() );
                }
            }
        }
        res.push(row);
    }
    res
}
