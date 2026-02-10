mod config;
use config::{Config, config_file_exists};

use hteapot::{Hteapot, HttpMethod, HttpRequest, HttpResponse, TunnelResponse, headers};

fn main() {
    let config = if config_file_exists("config.cfg") {
        println!("Loading configuration from file...");
        Config::from_file("config.cfg")
    } else {
        println!("Loading configuration from command line arguments...");
        Config::from_args()
    };
    let server = Hteapot::new_threaded("0.0.0.0", config.port, 3);
    server.listen(move |req: HttpRequest| {
        println!("New request to {} {}!", req.method.to_str(), &req.path);
        if config.auth.is_some() {
            let headers = headers!("Proxy-Authenticate" => "Basic");
            if let Some(proxy_auth) = req.headers.get("Proxy-Authorization") {
                if config.auth_basic().unwrap() != *proxy_auth {
                    println!("Invalid proxy authentication");
                    return HttpResponse::new(
                        hteapot::HttpStatus::ProxyAuthenticationRequired,
                        "",
                        headers,
                    );
                }
            } else {
                println!("No proxy authentication provided");
                return HttpResponse::new(
                    hteapot::HttpStatus::ProxyAuthenticationRequired,
                    "",
                    headers,
                );
            }
        }
        if req.method == HttpMethod::CONNECT {
            TunnelResponse::new(&req.path)
        } else {
            println!("{:?}", req);
            let addr = req.headers.get("host");
            let addr = if let Some(addr) = addr {
                addr
            } else {
                return HttpResponse::new(
                    hteapot::HttpStatus::BadRequest,
                    "Missing host header",
                    None,
                );
            };
            let response = req.brew(addr);
            match response {
                Ok(response) => response,
                Err(e) => {
                    println!("Error: {}", e);
                    HttpResponse::new(
                        hteapot::HttpStatus::InternalServerError,
                        "Unknown error",
                        None,
                    )
                }
            }
        }
    });
}
