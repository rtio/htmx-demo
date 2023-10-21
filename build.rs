fn main() {
    println!(r"cargo:rustc-link-search=/opt/homebrew/Cellar/vips/8.14.5_1/lib");
    println!(r"cargo:rustc-link-search=/opt/homebrew/Cellar/glib/2.78.0/lib");
    println!(r"cargo:rustc-link-search=/opt/homebrew/opt/gettext/lib");
}
