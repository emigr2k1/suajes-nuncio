use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};
use walkdir::WalkDir;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let templates_dir = Path::new(
        args.get(1)
            .expect("First argument must be templates directory"),
    );
    let partials_dir = templates_dir.join("partials");
    let partials_glob = templates_dir.join("partials/**/*");
    let out_dir = Path::new(args.get(2).expect("Second argument must be out directory"));

    let mut engine = Tera::new(partials_glob.to_str().unwrap()).unwrap();

    for entry in WalkDir::new(templates_dir)
        .into_iter()
        .filter_entry(|e| e.path().parent().unwrap() != &partials_dir)
    {
        let entry = entry.unwrap();
        let ext = entry.path().extension();
        if ext.is_some() && ext.unwrap() == "html" {
            let tmpl = std::fs::read_to_string(entry.path()).unwrap();
            let rendered = engine.render_str(&tmpl, &Context::new()).unwrap();

            let relative_path = entry
                .path()
                .parent()
                .unwrap()
                .strip_prefix(templates_dir)
                .unwrap();

            let file_out_path = out_dir.join(relative_path).join(entry.file_name());
            let file_out_dir = out_dir.join(relative_path);

            std::fs::create_dir_all(file_out_dir).unwrap();

            let mut out_file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_out_path)
                .unwrap();

            write!(&mut out_file, "{}", rendered).unwrap();
        }
    }
}
