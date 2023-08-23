use std::{
    fs,
    str,
    path::PathBuf,
};

use clap::{Arg, command, value_parser, ArgAction};
use object::Object;
use is_terminal::IsTerminal;
use console::style;
use termtree::Tree;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("file")
            .required(true)
            .action(ArgAction::Append)
            .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

    let files = matches.get_many::<PathBuf>("file")
        .unwrap_or_default().collect::<Vec<_>>();

    for file in files {
        let bin_data = fs::read(file)
            .expect("Could not read file");
        let obj_file = object::File::parse(&*bin_data)
            .expect("Could not parse file");
        let exports = obj_file.exports()
            .expect("Could not retrieve exports from object file");

        if std::io::stdout().is_terminal() {
            let mut root = Tree::new(format!("{}", file.canonicalize().expect("canonicalize").display()));
            for export in exports {
                root.push(Tree::new(format!("0x{:08x}: {}", export.address(), str::from_utf8(export.name()).unwrap())));
            }
            println!("{}", root);
        } else {
            for export in exports.iter().take(exports.len() - 1) {
                println!("0x{:08x}: {}", export.address(), str::from_utf8(export.name()).unwrap());
            }
            let last = exports.last().unwrap();
            print!("0x{:08x}: {}", last.address(), str::from_utf8(last.name()).unwrap());
        }
    }
}
