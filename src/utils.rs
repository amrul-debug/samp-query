//! Utility functions for the SAMP Query library.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

/// Converts a string to a socket address.
///
/// # Examples
///
/// ```
/// use samp_query::utils::parse_address;
///
/// let addr = parse_address("127.0.0.1:7777").unwrap();
/// assert_eq!(addr.ip().to_string(), "127.0.0.1");
/// assert_eq!(addr.port(), 7777);
/// ```
pub fn parse_address(address: &str) -> Result<SocketAddr, String> {
    if let Ok(addr) = address.parse::<SocketAddr>() {
        return Ok(addr);
    }

    if let Ok(ip) = address.parse::<IpAddr>() {
        return Ok(SocketAddr::new(ip, 7777));
    }

    if let Some(idx) = address.rfind(':') {
        let (_host, port_str) = address.split_at(idx);
        let port_str = &port_str[1..];

        if let Ok(port) = port_str.parse::<u16>() {
            // In a real implementation, you would perform DNS resolution here
            return Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port));
        }
    }

    // in a real implementation, you would perform DNS resolution
    Ok(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        7777,
    ))
}

/// Formats a duration as a human-readable string.
///
/// # Examples
///
/// ```
/// use samp_query::utils::format_duration;
/// use std::time::Duration;
///
/// let duration = Duration::from_millis(1500);
/// assert_eq!(format_duration(&duration), "1.5s");
/// ```
pub fn format_duration(duration: &Duration) -> String {
    let millis = duration.as_millis();

    if millis < 1000 {
        format!("{}ms", millis)
    } else if millis < 60000 {
        format!("{:.1}s", millis as f64 / 1000.0)
    } else {
        let minutes = millis / 60000;
        let seconds = (millis % 60000) / 1000;
        format!("{}m {}s", minutes, seconds)
    }
}

/// Measures the execution time of a function.
///
/// # Examples
///
/// ```
/// use samp_query::utils::measure_time;
///
/// let (result, duration) = measure_time(|| {
///     // Some computation
///     42
/// });
///
/// assert_eq!(result, 42);
/// println!("Execution time: {:?}", duration);
/// ```
pub fn measure_time<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();

    (result, duration)
}

/// Formats a byte size as a human-readable string.
///
/// # Examples
///
/// ```
/// use samp_query::utils::format_bytes;
///
/// assert_eq!(format_bytes(1024), "1.0 KiB");
/// assert_eq!(format_bytes(1500000), "1.4 MiB");
/// ```
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} KiB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.1} MiB", bytes as f64 / MB as f64)
    } else {
        format!("{:.1} GiB", bytes as f64 / GB as f64)
    }
}
