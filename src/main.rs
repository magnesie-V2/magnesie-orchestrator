mod services;

use services::photogrammetry_service::PhotogrammetryService;

#[allow(dead_code)]
fn main() {
    PhotogrammetryService::test(String::from("8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io"));
}

