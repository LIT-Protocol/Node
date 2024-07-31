use ethers::prelude::*;
use std::fs::{read_dir, write};
fn main() {
    // process lit contracts
    let result = read_dir("../lit-blockchain/abis/");

    if result.is_err() {
        return;
    }

    let files = result.unwrap();
    for file in files.flatten() {
        let file_path = file.path().canonicalize().unwrap();
        let abi_source = file_path.to_str().unwrap();
        let result = Abigen::from_file(abi_source);

        match result {
            Ok(res) => {
                let result = res.add_derive("serde::Serialize");
                let result = result.unwrap();
                let result = result.add_derive("serde::Deserialize");
                let result = result.unwrap();

                if let Ok(bindings) = result.generate() {
                    let source_file_name =
                        format!("../lit-blockchain/src/contracts/{:}.rs", &bindings.module_name());

                    // replace absolute path with relative
                    let as_str = bindings.to_string();
                    let as_str = as_str
                        .replace(abi_source, file.path().to_str().unwrap())
                        .replace("../lit-blockchain/abis/", "../../abis/");

                    write(source_file_name, as_str).expect("Could not write file.");
                }
            }
            Err(..) => {
                println!("Error generating ABI for {:?}:  {:?}", file_path, result.unwrap_err());
            }
        }
    }

    // // process other ABIs for node
    // let result = read_dir("../../lit-node/abis/");

    // if result.is_err() {
    //     return;
    // }

    // let files = result.unwrap();
    // for file in files.flatten() {
    //     let file_path = file.path().canonicalize().unwrap();
    //     let abi_source = file_path.to_str().unwrap();
    //     let result = Abigen::from_file(abi_source);

    //     match result {
    //         Ok(res) => {
    //             let result = res.add_derive("serde::Serialize");
    //             let result = result.unwrap();
    //             let result = result.add_derive("serde::Deserialize");
    //             let result = result.unwrap();

    //             if let Ok(bindings) = result.generate() {
    //                 let source_file_name =
    //                     format!("../../lit-node/src/contracts/{:}.rs", &bindings.module_name());

    //                 // replace absolute path with relative
    //                 let as_str = bindings.to_string();
    //                 let as_str = as_str
    //                     .replace(abi_source, file.path().to_str().unwrap())
    //                     .replace("../lit-node/abis/", "../abis/");

    //                 write(source_file_name, as_str).expect("Could not write file.");

    //                 // this used to be used to write it with the absolute path.  this is a problem
    //                 // because then it only works on the computer that genenerated the contracts.
    //                 // so, we now use the above code to replace the absolute path with a relative
    //                 // bindings.write_to_file(source_file_name).expect("Could not write file.");
    //             }
    //         }
    //         Err(..) => {
    //             println!("Error generating ABI for {:?}:  {:?}", file_path, result.unwrap_err());
    //         }
    //     }
    // }
}
