fn main() {
    let hachiko = "忠犬ハチ公".to_string();
    println!("{}", hachiko.slice(0, 1));
}

trait CanSlice {
    fn slice(&self, range_start: i32, range_end: i32) -> String;
}

impl CanSlice for String {
    fn slice(&self, range_start: i32, range_end: i32) -> String {
        let mut range_counter = 0;
        let mut string_buffer: String = "".to_string();

        for c in self.to_string().chars() {

            if range_counter < range_start {
                range_counter += 1;
                continue;
            }

            if range_counter > range_end {
                break;
            }

            string_buffer.push_str(&*c.to_string());

            range_counter += 1;
        }

        return string_buffer
    }
}

/*
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
*/
