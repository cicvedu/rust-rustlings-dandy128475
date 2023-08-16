// drive5.rs
// Your task is to make the testcase be able to call the `my_demo_function` in module Foo.
// the `my_demo_function_alias` is an alias for `my_demo_function_alias`, so the two line of
// code in the testcase should call the same function.
// You should not modify any existing code. All you need to do is add two line of attributes.





/*extern {
    fn my_demo_function(a:u32) -> u32;
    fn my_demo_function_alias(a:u32) -> u32;
}




/*mod Foo{
    fn my_demo_function(a:u32) -> u32 {a}
}*/
mod Foo {
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}


/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            #[path = "drive4.rs"]
            my_demo_function(123);

            #[path = "drive4.rs"]
            my_demo_function_alias(456);
        }
    }
}*/
// drive5.rs


mod my_module;

mod Foo {
    pub use crate::my_module::my_demo_function as my_demo_function_alias;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_module::my_demo_function;

    #[test]
    fn test_success() {
        unsafe {
            let result1 = my_demo_function(123);
            let result2 = Foo::my_demo_function_alias(456);

            assert_eq!(result1, 123);
            assert_eq!(result2, 456);
        }
    }
}




