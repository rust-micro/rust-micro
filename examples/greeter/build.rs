fn main() {
    micro::proto::configure()
        .compile(&["proto/echo.proto"], &["proto"])
        .unwrap();
}
