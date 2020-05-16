
fn main() {
    #[cfg(all(feature = "sqlite", feature = "mysql"))]
    compile_error!("Can't enable both sqlite and mysql at the same time");
    #[cfg(all(feature = "sqlite", feature = "postgres"))]
    compile_error!("Can't enable both sqlite and postgresql at the same time");
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    compile_error!("Can't enable both mysql and postgresql at the same time");

    #[cfg(not(any(feature = "sqlite", feature = "mysql", feature = "postgres")))]
    compile_error!("You need to enable one database backend. To build with defaults do: cargo build --features mysql");
}
