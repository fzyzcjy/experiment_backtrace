use std::env;

use anyhow::{anyhow, Result};

fn main() {
    println!("main start");
    env::set_var("RUST_BACKTRACE", "1");
    let err = simple_recursive_function().err().unwrap();
    println!("The error object I see: {:?}", err);
    println!("err.backtrace={:?}", err.backtrace());
    println!("main end");
}

pub fn simple_recursive_function() -> Result<()> {
    if rand::random::<f32>() < 0.9 {
        simple_recursive_function()?;
        simple_recursive_function()?;
        Ok(())
    } else {
        // print the backtrace
        println!("------------print backtrace------------");
        backtrace::trace(|frame| {
            println!("backtrace::trace frame={:?}", frame);
            backtrace::resolve_frame(frame, |symbol| {
                println!("backtrace::resolve_frame symbol={:?}", symbol);
            });
            true // keep going to the next frame
        });
        println!("---------------------------------------");

        // return anyhow error
        Err(anyhow!("deliberate error!"))
    }
}
