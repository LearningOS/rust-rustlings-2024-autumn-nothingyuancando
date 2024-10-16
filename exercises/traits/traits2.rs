// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// I AM 

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {

    fn append_bar(self) -> Self {  
        let bar = String::from("Bar");  
        // 使用into_iter()和collect()是不必要的，因为我们可以直接push。  
        // 但为了展示如何使用迭代器（尽管在这个特定情况下不是最优的），  
        // 我将保留这个实现，并在注释中指出更简单的方法。  
        // let new_vec: Vec<String> = self.into_iter().chain(std::iter::once(bar)).collect();  
          
        // 更简单且效率更高的方法：  
        let mut new_vec = self; // 消耗self并将其内容移动到new_vec中  
        new_vec.push(bar); // 直接在new_vec上调用push方法  
        new_vec 
    } 
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
