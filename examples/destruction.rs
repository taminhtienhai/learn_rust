struct StructInfo {
    pub name: String,
    pub size: u8,
    pub locate: u8,
}


fn main() {
    let info = &StructInfo { name: String::from(""), size: 10, locate: 102 };

    let StructInfo { 
        ref name,
        size,
        ..
    } = *info;


    let StructInfo { ref name, size, .. } = *info;


    println!("{name}");
}