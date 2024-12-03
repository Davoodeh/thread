// TODO remove
// #[test]
// fn test_syn() {
//     thread::te!(test.method::<X>);
// }

#[test]
fn test_single_instruction_single_arg_first() {
    let x = "hello";

    assert_eq!(
        thread::thread!(x first in ToString::to_string),
        "hello".to_string(),
    );
}

#[test]
fn test_single_instruction_single_arg_first_missing() {
    let x = "hello";

    assert_eq!(
        thread::thread!(x in ToString::to_string),
        "hello".to_string(),
    );
}

#[test]
fn test_single_instruction_single_arg_last() {
    let x = "hello";

    assert_eq!(
        thread::thread!(x last in ToString::to_string),
        "hello".to_string(),
    );
}

#[test]
fn test_single_instruction_single_arg_as() {
    let x = "hello";

    assert_eq!(
        thread::thread!(let v = x in ToString::to_string(v)),
        "hello".to_string(),
    );
}
