fn increase(num: &str) -> &str {
    return num;
}

fn decrese(num: &str) -> &str {
    return num;
}


fn cls_return_cls(a: String) -> impl Fn(String) -> String {
    return move |b: String| format!("{}{}",a,b).to_string();
}

fn closure(a: String) -> impl Fn(String) -> String {
    return move |b: String| format!("{}{}",a,b).to_string();
}

fn main() {
    let closure_01 = |a: String| move |b: String| format!("{}{}",a,b).to_string();
    let closure_02 = |a: String| move |b: String| format!("{}{}",a,b).to_string();
    let fnc_01 = closure_01(String::from("c"));
    let fnc_02 = |b: String| format!("{}",b).to_string();

    // let fn_arr = [fnc_01, fnc_02];

    let adders: Vec<_> = (1..10)
        .map(|a: i32| move |b: i32| a + b)
        .collect();

    let arr = adders;

    let json = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;


    // arr.push(cls_return_cls_02("a"));
}