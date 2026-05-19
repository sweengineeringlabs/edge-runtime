//! `RingBuffer` — fixed-capacity circular buffer for latency samples.

/// Ring buffer of latency samples in microseconds.
pub struct RingBuffer {
    pub(crate) buf: Vec<u64>,
    pub(crate) head: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity],
            head: 0,
        }
    }

    pub fn push(&mut self, val_us: u64) {
        let cap = self.buf.len();
        self.buf[self.head % cap] = val_us;
        self.head = self.head.wrapping_add(1);
    }

    /// 99th-percentile latency in milliseconds from the current window.
    pub fn p99_ms(&self) -> f64 {
        let mut samples: Vec<u64> = self.buf.iter().copied().filter(|&v| v > 0).collect();
        if samples.is_empty() {
            return 0.0;
        }
        samples.sort_unstable();
        let idx = (samples.len() * 99 / 100).saturating_sub(1);
        samples[idx] as f64 / 1_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: p99_ms
    #[test]
    fn test_ring_buffer_p99_ms_returns_correct_percentile() {
        let mut rb = RingBuffer::new(100);
        for i in 1u64..=100 {
            rb.push(i * 1_000);
        }
        let p99 = rb.p99_ms();
        assert!((98.0_f64..=100.0).contains(&p99), "p99={p99}");
    }

    /// @covers: p99_ms
    #[test]
    fn test_ring_buffer_p99_ms_returns_zero_when_empty() {
        let rb = RingBuffer::new(64);
        assert_eq!(rb.p99_ms(), 0.0);
    }

    /// @covers: push
    #[test]
    fn test_push_wraps_around_ring() {
        let mut rb = RingBuffer::new(4);
        for i in 0u64..8 {
            rb.push(i * 1_000);
        }
        assert!(rb.p99_ms() > 0.0);
    }
}
