mod handler;
mod server;
mod router;

use server::Server;

fn main() {
    let server = Server::new("loaclhost:3030");
    server.run();
}
