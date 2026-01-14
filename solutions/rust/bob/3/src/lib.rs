pub fn is_yell(message:&str)->bool{
    message.chars().any(|c| c.is_alphabetic()) 
                  && message.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase)
}

pub fn is_question(message:&str)->bool{
    message.trim().ends_with('?') 
}

pub fn reply(message: &str) -> &str {
    if message.trim().is_empty(){
            "Fine. Be that way!"
        }
    else if is_yell(message){
        if is_question(message){
            "Calm down, I know what I'm doing!"
        }
        else{
            "Whoa, chill out!"
        }
    }
    else {
        if is_question(message){
            "Sure."
        }
        else{
            "Whatever."
        }
    }
    
}
