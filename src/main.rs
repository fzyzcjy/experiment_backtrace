use anyhow::{anyhow, Result};

fn main() {
    println!("main start");
    let err = simple_recursive_function().err().unwrap();
    println!("err={:?}", err);
    println!("main end");
}

pub fn simple_recursive_function() -> Result<()> {
    if rand::random::<f32>() < 0.8 {
        simple_recursive_function()?;
        simple_recursive_function()?;
        Ok(())
    } else {
        // print the backtrace
        backtrace::trace(|frame| {
            println!("backtrace::trace frame={:?}", frame);
            backtrace::resolve_frame(frame, |symbol| {
                println!("backtrace::resolve_frame symbol={:?}", symbol);
            });
            true // keep going to the next frame
        });

        // return anyhow error
        Err(anyhow!("deliberate error!"))
    }
}
