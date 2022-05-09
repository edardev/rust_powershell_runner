use powershell_script;

// Creates a shortcut to notpad on the desktop
fn main() {
    let create_shortcut = include_str!("../script.ps");
    match powershell_script::run(create_shortcut) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
