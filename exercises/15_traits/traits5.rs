#[allow(dead_code)]
trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

#[allow(dead_code)]
struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
#[allow(dead_code)]
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
