// MIT/Apache2 License

use breadx::DisplayConnection;
use std::env;

fn main() -> breadx::Result {
    env::set_var("RUST_LOG", "breadx=debug");
    env_logger::init();

    let mut conn = DisplayConnection::create(None, None)?;
    let _win = conn.create_simple_window(
        conn.default_screen().root,
        0,
        0,
        640,
        400,
        0,
        conn.default_black_pixel(),
        conn.default_white_pixel(),
    )?;

    let cfg = conn.get_visual_configs_immediate(conn.default_screen_index())?;
    println!("Num props per visual: {}", cfg.num_properties_per_config);
    println!("Num visuals: {}", cfg.num_configs);
    println!(
        "Est. total size: {}",
        cfg.num_properties_per_config * cfg.num_configs
    );
    cfg.properties.iter().enumerate().for_each(|(i, prop)| {
        println!("[{}] {}", i, prop);
    });

    /*let cfg = conn.get_fb_configs_immediate(conn.default_screen_index())?;
    println!("Num props per fbc: {}", cfg.num_properties_per_config);
    println!("Num fbcs: {}", cfg.num_configs);
    println!("Est. total size: {}", cfg.num_properties_per_config * cfg.num_configs);
    cfg.properties.iter().enumerate().for_each(|(i, prop)| {
        println!("[{}] {}", i, prop);
    });*/

    loop {
        let ev = conn.wait_for_event()?;
        println!("{:?}", ev);
    }

    Ok(())
}
