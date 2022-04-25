mod handler;
mod server;
mod router;
use server::Server;
fn main() {
    // Start a server
    let server = Server::new("localhost:9000");
    //Run the server
    server.run();
}
