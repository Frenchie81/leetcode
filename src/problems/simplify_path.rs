// problem 71
struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<String> = Vec::new();

        let mut buffer = String::new();
        for c in path.chars() {
            match c {
                '/' => Solution::add_to_stack(&mut stack, &mut buffer),
                _ => buffer.push(c),
            }
        }

        if !buffer.is_empty() {
            Solution::add_to_stack(&mut stack, &mut buffer);
        }

        "/".to_owned() + stack.join("/").as_str()
    }

    fn add_to_stack(stack: &mut Vec<String>, buffer: &mut String) {
        if !buffer.is_empty() {
            match buffer.as_str() {
                "." => buffer.clear(),
                ".." => _ = stack.pop(),
                _ => stack.push(buffer.clone()),
            }
            buffer.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let path = "/home/".to_owned();

        let output = Solution::simplify_path(path);

        assert_eq!("/home", output);
    }

    #[test]
    fn example2() {
        let path = "/home//foo/".to_owned();

        let output = Solution::simplify_path(path);

        assert_eq!("/home/foo", output);
    }
    #[test]
    fn example3() {
        let path = "/home/user/Documents/../Pictures".to_owned();

        let output = Solution::simplify_path(path);

        assert_eq!("/home/user/Pictures", output);
    }
    #[test]
    fn example4() {
        let path = "/../".to_owned();

        let output = Solution::simplify_path(path);

        assert_eq!("/", output);
    }
    #[test]
    fn example5() {
        let path = "/.../a/../b/c/../d/./".to_owned();

        let output = Solution::simplify_path(path);

        assert_eq!("/.../b/d", output);
    }
}
