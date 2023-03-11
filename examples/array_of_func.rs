fn increase(num: &str) -> &str {
    return num;
}

fn decrese(num: &str) -> &str {
    return num;
}

fn cls_return_cls(a: String) -> impl Fn(String) -> String {
    return move |b: String| format!("{}{}",a,b).to_string();
}

fn cls_return_cls_02(a: String) -> impl Fn(String) -> String {
    return move |b: String| format!("{}{}",a,b).to_string();
}

fn main() {
    let mut arr = Vec::from_iter([
        cls_return_cls(String::from("c")),
        cls_return_cls(String::from("a")),
    ]);

    arr.push(cls_return_cls(String::from("b")));

    let result = arr.iter().map(|item| item(String::from("b"))).collect::<Vec<_>>();

    println!("{result:?}");
}