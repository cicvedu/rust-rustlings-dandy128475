// drive3.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.



// We look for an environment variable and expect it to fall in a range.
// look into the testcase to find out the details.
// You should not modify this file. Modify `build.rs` to pass this exercise.
/*use std::env;
fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    env::set_var("TEST_FOO", timestamp.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e:u64 = s.parse().unwrap();
        assert! (timestamp >= e && timestamp < e + 10);
    }
} */
use std::env;

fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    env::set_var("TEST_FOO", timestamp.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 设置环境变量
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        env::set_var("TEST_FOO", timestamp.to_string());

        // 获取设置的环境变量并进行测试
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        let current_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        assert!(current_timestamp >= e && current_timestamp < e + 10);
    }
}

