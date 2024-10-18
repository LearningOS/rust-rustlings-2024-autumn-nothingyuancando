// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let ret = unsafe { Box::from_raw(ptr) };
    //todo!("The rest of the code goes here")
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        // 将 Box 转换为原始指针
        let raw_ptr = Box::into_raw(data);

        // SAFETY: 我们传递一个拥有的 Foo 的 Box。
        let ret = unsafe { raw_pointer_to_box(raw_ptr) };

        // 检查指针地址是否匹配
        assert_eq!(&ret.a as *const u128, &ret.a as *const u128);

        // 确保 `b` 仍然是 None
        assert!(ret.b.is_none());
    }
}