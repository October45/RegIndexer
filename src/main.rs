use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hku = RegKey::predef(HKEY_USERS);
    let hkcc = RegKey::predef(HKEY_CURRENT_CONFIG);
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    println!("HKEY_LOCAL_MACHINE:");
    traverse_registry(&hklm, 0)?;

    println!("HKEY_CURRENT_USER:");
    traverse_registry(&hkcu, 0)?;

    println!("HKEY_CURRENT_USER:");
    traverse_registry(&hku, 0)?;

    println!("HKEY_CURRENT_USER:");
    traverse_registry(&hkcc, 0)?;

    println!("HKEY_CURRENT_USER:");
    traverse_registry(&hkcr, 0)?;

    Ok(())
}

fn traverse_registry(key: &RegKey, depth: usize) -> io::Result<()> {
    for (name, subkey) in key.enum_keys().filter_map(Result::ok).map(|name| (name.clone(), key.open_subkey(name))) {
        match subkey {
            Ok(subkey) => {
                println!("{:indent$}{}", "", name, indent = depth * 2);
                traverse_registry(&subkey, depth + 1)?;
            }
            Err(e) => {
                eprintln!("Error opening subkey: {}", e);
            }
        }
    }

    for (name, value) in key.enum_values().filter_map(Result::ok) {
        println!("{:indent$}{}: {:?}", "", name, value, indent = (depth + 1) * 2);
    }

    Ok(())
}