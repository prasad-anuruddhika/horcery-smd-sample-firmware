extern crate cc;

fn main()
{
    // Linking with a static library (.a file)
    println!("cargo:rustc-link-lib=static=bsecwrap");
    println!("cargo:rustc-link-search=native=lib"); // Relative path to the library directory
}