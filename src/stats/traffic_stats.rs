use std::time::Instant;

pub struct TrafficStats {
    pub packets: u64,
    pub bytes: u64,
    pub start_time: Instant,
    pub last_update: Instant,
    pub total_packets: u64,
    pub total_bytes: u64,
}

impl TrafficStats {
    pub fn new() -> Self {
        let now = Instant::now();
        TrafficStats {
            packets: 0,
            bytes: 0,
            start_time: now,
            last_update: now,
            total_packets: 0,
            total_bytes: 0,
        }
    }

    pub fn get_total(&self) -> (u64, u64) {
        (self.total_packets, self.total_bytes)
    }

    pub fn add_packet(&mut self, packet_len: usize) {
        self.packets += 1;
        self.bytes += packet_len as u64;
    }

    pub fn reset(&mut self) -> (f64, f64, u64, u64) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();

        if elapsed == 0.0 {
            return (0.0, 0.0, 0, 0);
        }

        let pps = self.packets as f64 / elapsed;
        let bps = self.bytes as f64 / elapsed;

        let packets = self.packets;
        let bytes = self.bytes;

        self.total_packets += self.packets;
        self.total_bytes += self.bytes;
        self.packets = 0;
        self.bytes = 0;
        self.last_update = now;

        (pps, bps, packets, bytes)
    }
}
