extern crate chrono;
extern crate fern;

// trace!("Trace message");
// debug!("Debug message");
// info!("Info message");
// warn!("Warning message");
// error!("Error message");

pub fn init() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout())
        .apply()
        .expect("Unable to initialize logger");
}
