fn first_input_value(a: &str, _: (), _: ()) -> &str {
    a
}

fn last_input_value(_: (), _: (), a: &str) -> &str {
    a
}

fn third_input_value(_: (), _: (), a: &str, _: (), _: (), _: ()) -> &str {
    a
}

#[test]
fn test_single_instruction_multi_arg_first() {
    let x = "hello";

    assert_eq!(
        thread::thread!(x first in first_input_value((), ())),
        "hello".to_string(),
    );
}

#[test]
fn test_single_instruction_multi_arg_first_missing() {
    let x = "hello";

    assert_eq!(
        thread::thread!(x in first_input_value((), ())),
        "hello".to_string(),
    );
}

#[test]
fn test_single_instruction_multi_arg_last() {
    let x = "hello";

    assert_eq!(
        thread::thread!(x last in last_input_value((), ())),
        "hello".to_string(),
    );
}

#[test]
fn test_single_instruction_multi_arg_as() {
    let x = "hello";

    assert_eq!(
        thread::thread!(let a = x in third_input_value((), (), a, (), (), ())),
        "hello".to_string(),
    );
}
