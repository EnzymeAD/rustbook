#[macro_export]
macro_rules! test {
    ($m:ident; $($func:item)*) => {
        mod $m {
            $($func)*
            #[test]
            fn run() { main() }
        }
    };
}
