// use error_chain::error_chain;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

pub mod http_helper {
    use std::io::Read;
    use reqwest::blocking::Response;

    pub fn read_manifest() {
        let mut res: Response = reqwest::blocking::get("http://httpbin.org/get").unwrap();
        let mut body: String = String::new();
        res.read_to_string(&mut body).unwrap();

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!("Body:\n{}", body);
    }
}
