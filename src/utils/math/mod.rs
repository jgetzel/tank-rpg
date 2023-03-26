pub mod polygon;

pub fn seconds_to_formatted_time_string(time_remaining_int: u32) -> String {
    let seconds = time_remaining_int % 60;
    let minutes = (time_remaining_int / 60) % 60;
    let hours = time_remaining_int / 60 / 60;

    let seconds_string = if seconds < 10 {
        ["0", &seconds.to_string()].join("")
    } else { seconds.to_string() };
    let minutes_string = if minutes < 10 && hours > 0 {
        ["0", &minutes.to_string()].join("")
    } else { minutes.to_string() };

    let minutes_seconds_string = format!("{}:{}", minutes_string, seconds_string);
    
    if hours > 0 {
        format!("{}:{}", hours, minutes_seconds_string)
    } else { minutes_seconds_string }
}
