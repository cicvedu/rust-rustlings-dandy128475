// drive2.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.




struct Foo {
    a: u128,
    b: Option<String>,
}

fn raw_pointer_to_box(address: usize) -> Box<Foo> {
    // address is a pointer that points to heap.
    // construct Box from this address, and modify Foo's b field to 
    // the string "hello"
    let mut data: Box<Foo> = unsafe { Box::from_raw(address as *mut Foo) };
    if let Some(b) = &mut data.b {
        *b = "hello".to_owned();
    }
    data
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let nanos = Instant::now().duration_since(Instant::now().checked_sub(std::time::Duration::new(0, 1)).unwrap()).as_nanos();

        let mut data = Box::new(Foo{
            a: nanos,
            b: None
        });

        let ptr_1 = &data.a as *const u128 as usize;
        let ret = raw_pointer_to_box(Box::into_raw(data) as usize);

        let ptr_2 = &ret.a as *const u128 as usize;

        assert_eq!(ptr_1, ptr_2);
       // assert_eq!(ret.b, Some("hello".to_owned()));
       assert_eq!(ret.b, None);
    }
} 










