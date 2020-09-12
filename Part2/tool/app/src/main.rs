fn main() {
    library::serde();
    // library::special();

    if cfg!(feature = "special") {
        println!("success: special");
    }
}
