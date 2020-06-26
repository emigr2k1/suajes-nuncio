fn main() {
    let mut args = std::env::args();
    let dir = std::fs::read_dir(&args.nth(1).expect("first argument must be dir of imgs")).expect("dir doesn't exist");

    for file in dir {
        let file = file.unwrap();

        let metadata = file.metadata().unwrap();

        if metadata.is_file() && file.file_name().into_string().unwrap().ends_with("resized.jpg") {
            let img = image::open(file.path()).unwrap();

            let mut path = file.path();

            let name = path.file_name().unwrap().to_str().unwrap();
            let name = name[0..name.len()-3].to_string();

            //// resized image
            //let resized_img = img.resize(2000, 2000, image::imageops::FilterType::Triangle);
            //let resized_img_name = name.clone() + "-resized.jpg";
            //path.set_file_name(&resized_img_name);

            //resized_img.save(&path).unwrap();

            // thumbnail
            let thumbnail = img.thumbnail(600, 600);
            let thumbnail = thumbnail.thumbnail_exact(300, 300);
            let thumbnail_name = name + "-thumbnail.jpg";
            path.set_file_name(thumbnail_name);

            thumbnail.save(&path).unwrap();

        }

    }
}
