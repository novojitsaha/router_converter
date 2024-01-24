mod converter;

fn main() {
    let file_path = String::from("./router_files/test.txt");

    match converter::read_file(&file_path) {
        Ok(contents) => println!("File contents: \n{}", contents ),
        Err(e) => println!("Error reading file: {:?}", e)
    }


}
