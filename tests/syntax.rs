fn only_first(s: &str, _: ()) -> String {
    s.to_string()
}

fn only_last(_: (), s: &str) -> String {
    s.to_string()
}

#[cfg(test)]
mod initial_expr {
    #[test]
    fn literal_string() {
        assert_eq!(
            thread::thread!("hello" in ToString::to_string),
            "hello".to_string()
        );
    }

    #[test]
    fn ident() {
        let x = "hello";
        assert_eq!(
            thread::thread!(x in ToString::to_string),
            "hello".to_string()
        );
    }
}

#[cfg(test)]
mod keywords {
    use super::only_first;

    #[test]
    fn first() {
        assert_eq!(
            thread::thread!("hello" first in only_first(())),
            "hello".to_string(),
        );
    }

    #[test]
    fn optional_first() {
        assert_eq!(
            thread::thread!("hello" in only_first(())),
            "hello".to_string(),
        );
    }

    #[test]
    fn last() {
        assert_eq!(
            thread::thread!("hello" last in {|_: (), s: &str| s.to_string()}(()) ),
            "hello".to_string(),
        );
    }

    // #[test]
    // fn as_magic() {
    //     assert_eq!(
    //         thread::thread!(let x = "hello" in {|_: (), s: &str| s.to_string()}((), x) ),
    //         "hello".to_string(),
    //     );
    // }
}

#[cfg(test)]
mod magic {

    #[cfg(test)]
    mod some {
        use crate::{only_first, only_last};

        #[test]
        fn first() {
            let v = Some("hello");
            assert_eq!(
                thread::thread!(Some(v) first in only_first(())),
                Some("hello".to_string()),
            );

            assert_eq!(thread::thread!(Some(None) first in only_first(())), None);
        }

        #[test]
        fn optional_first() {
            let v = Some("hello");
            assert_eq!(
                thread::thread!(Some(v) in only_first(())),
                Some("hello".to_string()),
            );

            assert_eq!(thread::thread!(Some(None) in only_first(())), None);
        }

        #[test]
        fn last() {
            let v = Some("hello");
            assert_eq!(
                thread::thread!(Some(v) last in only_last(())),
                Some("hello".to_string()),
            );

            assert_eq!(thread::thread!(Some(None) last in only_last(())), None);
        }

        #[test]
        fn as_magic() {
            // let v = Some("hello");
            // assert_eq!(
            //     thread::thread!(let Some(x) = "hello" in { |_: (), s: &str| s.to_string() }((), x) ),
            //     "hello".to_string(),
            // );

            assert_eq!(thread::thread!(Some(None) last in only_last(())), None);
        }
    }
}

#[cfg(test)]
mod punctuations {
    #[test]
    fn trailing_comma() {
        assert_eq!(
            thread::thread!("hello" first in ToString::to_string,),
            "hello".to_string(),
        );
    }

    #[test]
    fn no_trailing_comma() {
        assert_eq!(
            thread::thread!("hello" first in ToString::to_string),
            "hello".to_string(),
        );
    }
}

#[cfg(test)]
mod pipeline_func {
    #[test]
    fn symbol() {
        let f = |s: &str| s.to_string();
        assert_eq!(thread::thread!("hello" first in f), "hello".to_string());
    }

    #[test]
    fn path() {
        assert_eq!(
            thread::thread!("hello" in ToString::to_string),
            "hello".to_string()
        );
    }

    #[test]
    fn closure() {
        assert_eq!(
            thread::thread!("hello" first in |s: &str| s.to_string()),
            "hello".to_string()
        );
    }
}

#[cfg(test)]
mod composition {
    // FIXME how to test Syntax panics?
    // #[test]
    // fn no_empty_pipe() {
    //     thread::thread!("hello" first in);
    // }

    #[test]
    fn closure_and_ident() {
        // Not that this is a logical thing to do, it's just to check the syntax
        let f = |s: String| s.len();
        assert_eq!(
            thread::thread!("hello" first in
                            |s: &str| s.split_once('l').unwrap().1.to_string(),
                            f,
            ),
            2
        );
    }

    #[test]
    fn macro_call() {
        assert_eq!(
            thread::thread!("hello" last in {|_: (), s: &str| s.to_string()}(()) ),
            "hello".to_string(),
        );
    }
}
