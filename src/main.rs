extern crate ws;

use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode, Error};

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
}

impl Handler for Client {
	 fn on_error(&mut self, err: Error) { 
	     println!("on error [{}]", err);
	 }

    fn on_open(&mut self, hh: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        let version = hh.request.version();
        match version {
            Ok(version) => println!("on_open version {}", version),
            Err(_) => println!("on_open error")
        }

        self.out.send("Hello WebSocket")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
//        let y:i32 = 2;
//        let _x = move|y:i32| y*y;

//        println!("Foo {}", _x(3));

        match msg {
            Message::Text(str) => {
                if str.starts_with("Vol") {
                    println!("Msg {}", str);
                }
            },
            Message::Binary(_) => {}
        }
//        self.out.close(CloseCode::Normal)
        Ok(())
    }
}

fn main() {
  connect("ws://localhost:8080/websocket", |out| Client { out: out } ).unwrap()
}

