use std::thread::JoinHandle;
use std::io::{Read, Write};


fn pipe_thread<R, W>(mut r: R, mut w: W) -> JoinHandle<()>
where R: Read + Send + 'static,
      W: Write + Send + 'static
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {

            let len = r.read(&mut buffer).unwrap();
            if len == 0 {
                break;
            }
            w.write(&buffer[..len]).unwrap();
            w.flush().unwrap();

        }
    })
}


fn main() {
    let x = std::net::TcpListener::bind("0.0.0.0:3333").unwrap();
    let (stream, _) = x.accept().unwrap();
    let t1 = pipe_thread(std::io::stdin(), stream.try_clone().unwrap());
    let t2 = pipe_thread(stream, std::io::stdout());
    t1.join();
    t2.join();
}
