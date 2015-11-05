fn main() {

    let hachiko = "忠犬ハチ公".to_string();
    slice(hachiko, 0, 1);
}

fn slice(string: String, range_start: i32, range_end: i32) {
    
    let mut range_counter = 0;
    
    for c in string.chars() {
        
        if range_counter < range_start {
            range_counter += 1;
            continue;
        }
        
        if range_counter > range_end {
            break;
        }
        
        print!("{}, ", c);
        
        range_counter += 1;
    }
}
