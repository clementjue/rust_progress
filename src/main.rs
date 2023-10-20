struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars: Vec<char> = password.chars().collect();
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut replace_count: i32 = 0;
        let mut one_mod: i32 = 0;
        let mut two_mod: i32 = 0;
        
        let mut i = 0;
        while i < chars.len() {
            if chars[i].is_ascii_lowercase() {
                has_lower = true;
            }
            if chars[i].is_ascii_uppercase() {
                has_upper = true;
            }
            if chars[i].is_ascii_digit() {
                has_digit = true;
            }
            
            let start = i;
            while i < chars.len() && chars[i] == chars[start] {
                i += 1;
            }
            let length = i - start;
            
            if length >= 3 {
                replace_count += (length / 3) as i32;
                if length % 3 == 0 {
                    one_mod += 1;
                } else if length % 3 == 1 {
                    two_mod += 1;
                }
            }
        }
        
        let missing_types = [has_lower, has_upper, has_digit].iter().filter(|&&x| !x).count() as i32;
        
        if chars.len() < 6 {
            return std::cmp::max(missing_types, 6 - chars.len() as i32);
        } else if chars.len() <= 20 {
            return std::cmp::max(missing_types, replace_count);
        } else {
            let delete_count = chars.len() as i32 - 20;
            replace_count -= std::cmp::min(delete_count, one_mod) / 1;
            replace_count -= std::cmp::min(std::cmp::max(delete_count - one_mod, 0), two_mod * 2) / 2;
            replace_count -= std::cmp::max(delete_count - one_mod - 2 * two_mod, 0) / 3;
            
            return delete_count + std::cmp::max(missing_types, replace_count);
        }
    }
}

fn main() {
    let password1 = "a".to_string();
    let password2 = "aA1".to_string();
    let password3 = "1337C0d3".to_string();

    println!("{}", Solution::strong_password_checker(password1));  
    println!("{}", Solution::strong_password_checker(password2));  
    println!("{}", Solution::strong_password_checker(password3));  
}
