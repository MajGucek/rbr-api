mod data;

pub use data::*;

use std::{
    io::{self, ErrorKind},
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use log::error;

/*
 * recv() needs a timeout, otherwise the thread never gets to check if it should stop.
 */
const RECV_TIMEOUT: Duration = Duration::from_millis(100);

/// Receives NGP telemetry on its own thread until dropped.
///
/// NGP sends packets faster than the game ticks, so the callback runs on that thread,
/// hand the data over to your plugin with a Mutex or a channel.
pub struct TelemetryReceiver {
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl TelemetryReceiver {
    /// `port` is the telemetry port from the RSF launcher.
    pub fn spawn<F>(port: u16, callback: F) -> io::Result<Self>
    where
        F: FnMut(&Telemetry) + Send + 'static,
    {
        let socket = UdpSocket::bind(("127.0.0.1", port))?;
        socket.set_read_timeout(Some(RECV_TIMEOUT))?;

        let running = Arc::new(AtomicBool::new(true));
        let thread_running = running.clone();

        let thread = thread::Builder::new()
            .name("ngp-telemetry".into())
            .spawn(move || receive_loop(socket, thread_running, callback))?;

        Ok(Self {
            running,
            thread: Some(thread),
        })
    }
}

impl Drop for TelemetryReceiver {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);

        if let Some(thread) = self.thread.take()
            && thread.join().is_err()
        {
            error!("NGP telemetry thread panicked");
        }
    }
}

fn receive_loop<F>(socket: UdpSocket, running: Arc<AtomicBool>, mut callback: F)
where
    F: FnMut(&Telemetry),
{
    let mut buffer = [0u8; 2 * PACKET_SIZE];
    let mut warned = false;

    while running.load(Ordering::Relaxed) {
        let length = match socket.recv(&mut buffer) {
            Ok(length) => length,

            // timeout, or windows complaining about udp, just try again
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::ConnectionReset) => {
                continue;
            }

            Err(e) => {
                error!("NGP telemetry socket failed, stopping: {e}");
                return;
            }
        };

        match Telemetry::from_bytes(&buffer[..length]) {
            Some(telemetry) => callback(&telemetry),

            None if !warned => {
                warned = true;
                error!("Got a {length} byte packet, NGP telemetry is {PACKET_SIZE} bytes");
            }

            None => {}
        }
    }
}
