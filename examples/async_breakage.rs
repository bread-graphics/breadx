use std::time::Duration;
use breadx::{DisplayConnection, ImageFormat, Image, drawable };
use smol::{Executor, Timer, future::race};

// async entry point
async fn entry() -> breadx::Result<()> {
    const WIDTH: usize = 997;
    const HEIGHT: usize = 1009;
    const MEMORY: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

    let mut conn = DisplayConnection::create_async(None, None).await?;
    let image = Image::new(
        &conn,
        None,
        24,
        ImageFormat::ZPixmap,
        0,
        &MEMORY[..],
        WIDTH,
        HEIGHT,
        32,
        None,
    ).unwrap();
    let root = conn.default_root();
    let gc = conn.create_gc(root, Default::default())?;
    let pixmap = drawable::create_pixmap(&mut conn, root, WIDTH as _, HEIGHT as _, 24)?;
    for _ in 0..10usize {
        let put = async {
            drawable::put_image_async(
                &mut conn,
                pixmap,
                gc,
                &image,
                0,
                0,
                0,
                0,
                WIDTH as _,
                HEIGHT as _,
            ).await.unwrap();
            "PutImage"
        };
        let timeout = async {
            Timer::after(Duration::from_millis(5)).await;
            "timeout"
        };
        println!("{:?}", race(put, timeout).await);
    }
    Ok(())
}

fn main() -> breadx::Result<()> {
    //std::env::set_var("RUST_LOG", "breadx=trace");
    env_logger::init();
    let ex = Executor::new();
    smol::block_on(ex.run(entry())).unwrap();
    Ok(())
}
