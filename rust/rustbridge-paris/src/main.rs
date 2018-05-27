extern crate motivations;
extern crate rand;
extern crate simple_server as ss;

use motivations::MOTIVATIONS;
use rand::prelude::*;
use rand::Rng;
use ss::Server;

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        println!("Request received! {} {}", request.method(), request.uri());

        use ss::Method;
        match (request.method(), request.uri().path()) {
            (&Method::GET, "/hello") => {
                let choice = rand::thread_rng().gen_range(0, MOTIVATIONS.len());
                Ok(response.body(MOTIVATIONS[choice].as_bytes())?)
            }
            _ => Ok(response.body("Unsupported!".as_bytes())?)
        }
    });
    server.listen(host, port);
}
