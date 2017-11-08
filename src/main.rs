extern crate ftc_http;

fn main() {
    ftc_http::down("");
    ftc_http::clean();
    ftc_http::up("");
    ftc_http::build();
}
