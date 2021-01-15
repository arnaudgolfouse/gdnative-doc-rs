use godot_doc_rs::{documentation, files};

fn main() {
    let path = std::env::args_os().nth(1).unwrap();
    let package = files::Package::from_root_file(path.into()).unwrap();
    println!("{:#?}", package);
    println!();
    println!("===============================");
    println!("===============================");
    println!("===============================");
    println!();

    let mut documentation = documentation::Documentation::new();
    for (module_id, module) in package.modules {
        let root_module = if module_id == package.root_module {
            match module.attributes.as_ref() {
                Some(attrs) => Some(attrs.as_slice()),
                None => None,
            }
        } else {
            None
        };
        documentation
            .parse_from_module(&module.items, root_module)
            .unwrap();
    }
    println!("{:#?}", documentation)
}
