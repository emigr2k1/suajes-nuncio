fn main() {
    let prefix = std::env::args().nth(1).unwrap();
    let dir = std::fs::read_dir(std::env::args().nth(2).unwrap()).unwrap();
    let mut html = String::with_capacity(20000);

    html.push_str("<div id='galeria'>\n");

    for item in dir {
        let item = item.unwrap();
        let name = item.file_name().into_string().unwrap();
        let name = name[0..name.len()-3].to_string();

        if name.contains("thumbnail") {
            std::fs::remove_file(item.path()).unwrap();
        }

        if !name.contains("resized") && !name.contains("thumbnail") {
            html.push_str(&format!("    <img src='{}/{}-resized.jpg'>\n", prefix, name));
        }
    }

    html.push_str(r#"</div>"#);
    println!("{}", html);
}
