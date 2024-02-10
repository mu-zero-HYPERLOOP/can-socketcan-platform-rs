use can_socketcan_platform_rs::OwnedCanSocket;

fn main() {
    let can_adapter = OwnedCanSocket::open("can0").expect("failed to connect to can0");

    loop {
        match can_adapter.as_ref().receive() {
            Ok(frame) => {
                println!("{frame:?}");
            }
            Err(error) => println!("{error:?}"),
        }
    }
}
