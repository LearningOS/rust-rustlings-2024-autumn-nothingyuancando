// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

// I AM NOT 

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(&str,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output = Vec::new(); 
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let string_slice = string.to_string(); // 将字符串切片转换为String  
            let transformed = match command {  
                Command::Uppercase => string_slice.to_uppercase(),  
                Command::Trim => string_slice.trim().to_string(),  
                Command::Append(n) => {  
                    // 假设我们要追加'o'字符，但这里需要更明确的逻辑  
                    // 因为Append只接收了一个usize值，没有指定要追加的字符  
                    // 为了符合测试案例，我们临时假设要追加'o'  
                    let append_char = "bar";  
                    let appended = format!("{}{}", string_slice, &std::iter::repeat(append_char).take(*n).collect::<String>());  
                    appended  
                }  
            };  
            output.push(transformed);  

        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
