
pub fn is_flower(garden: &[&str], m:i16, n:i16, row_len:&i16, col_len:&i16)->bool{
    if m<0 || n<0{
        return false
    }
    if n  >= *col_len{
        return false
    }
    if m >= *row_len{
        return false
    }
    if garden[n as usize].as_bytes()[m as usize] == b'*'{
        return true
    }
    false
}
pub fn annotate(garden: &[&str]) -> Vec<String> {

    if garden.is_empty(){
        return vec![]
    }

    let mut res:Vec<String> = Vec::new();
    let col_len = garden.len();
    for n in 0..col_len{
        let mut row = String::new();
        let row_len = garden[n].len();
        for m in 0..row_len{
            if is_flower(garden, m as i16, n as i16, &(row_len as i16), &(col_len as i16)){
                row.push('*');
            }else{
                let mut count = 0;
                for i in -1..=1{
                    for j in -1..=1{
                        if i==0 && j==0{
                            continue
                        }
                        if is_flower(garden, (m as i16)+j, (n as i16)+i, &(row_len as i16), &(col_len as i16)){
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
