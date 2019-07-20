use fern::colors::{Color, ColoredLevelConfig};

pub fn set_up_logging() -> Result<(), String> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.info(Color::Green);
    let dispatch = fern::Dispatch::new()
        // stdout and stderr logging
        .level(log::LevelFilter::Trace)
        .chain({
            fern::Dispatch::new()
                .filter(|metadata| metadata.target().starts_with("wasm_game"))
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{color_line}[{level}{color_line}]{ansi_close} {message}",
                        color_line = format_args!(
                            "\x1B[{}m",
                            colors_line.get_color(&record.level()).to_fg_str()
                        ),
                        level = colors_level.color(record.level()),
                        ansi_close = "\x1B[0m",
                        message = message,
                    ));
                })
                .chain(std::io::stdout())
        });

    dispatch.apply().map_err(|e| e.to_string())?;

    Ok(())
}
