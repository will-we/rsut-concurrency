use std::thread;

fn main() -> anyhow::Result<()> {
    let handle = thread::spawn(|| {
        println!("hello world by the thread id {:?}!", thread::current().id());
    });

    println!("this is the main thread id {:?}!", handle.thread().id());

    // wait for the thread to finish
    handle.join().map_err(|e| anyhow::anyhow!("{:?}", e))?;

    Ok(())
}
