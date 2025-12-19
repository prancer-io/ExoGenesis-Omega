//! Lock-free ring buffer for real-time audio streaming

use ringbuf::{HeapRb, traits::{Consumer, Producer, Split}};
use parking_lot::Mutex;
use std::sync::Arc;
use crate::{Result, StreamError};

/// Thread-safe audio buffer using lock-free SPSC ring buffer
pub struct AudioBuffer {
    producer: Arc<Mutex<ringbuf::HeapProd<f32>>>,
    consumer: Arc<Mutex<ringbuf::HeapCons<f32>>>,
    capacity: usize,
}

impl AudioBuffer {
    /// Create a new audio buffer with specified capacity
    pub fn new(capacity: usize) -> Self {
        let ring = HeapRb::<f32>::new(capacity);
        let (producer, consumer) = ring.split();

        Self {
            producer: Arc::new(Mutex::new(producer)),
            consumer: Arc::new(Mutex::new(consumer)),
            capacity,
        }
    }

    /// Push samples into the buffer (called from audio callback)
    /// Returns number of samples actually written
    pub fn push(&self, samples: &[f32]) -> usize {
        let mut producer = self.producer.lock();
        let available = producer.free_len();

        if available < samples.len() {
            // Buffer overflow - only write what fits
            producer.push_slice(&samples[..available])
        } else {
            producer.push_slice(samples)
        }
    }

    /// Pop samples from the buffer (called from consumer thread)
    /// Returns number of samples actually read
    pub fn pop(&self, output: &mut [f32]) -> usize {
        let mut consumer = self.consumer.lock();
        consumer.pop_slice(output)
    }

    /// Get number of samples available to read
    pub fn available(&self) -> usize {
        self.consumer.lock().len()
    }

    /// Get number of free slots in buffer
    pub fn free_space(&self) -> usize {
        self.producer.lock().free_len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.consumer.lock().is_empty()
    }

    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.producer.lock().is_full()
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear the buffer
    pub fn clear(&self) {
        let mut consumer = self.consumer.lock();
        while !consumer.is_empty() {
            consumer.pop();
        }
    }

    /// Get buffer fill ratio (0.0 = empty, 1.0 = full)
    pub fn fill_ratio(&self) -> f32 {
        self.available() as f32 / self.capacity as f32
    }
}

impl Clone for AudioBuffer {
    fn clone(&self) -> Self {
        Self {
            producer: Arc::clone(&self.producer),
            consumer: Arc::clone(&self.consumer),
            capacity: self.capacity,
        }
    }
}

/// Statistics about buffer performance
#[derive(Debug, Clone, Copy, Default)]
pub struct BufferStats {
    pub overflows: u64,
    pub underflows: u64,
    pub total_samples: u64,
    pub peak_fill_ratio: f32,
    pub average_fill_ratio: f32,
}

/// Audio buffer with performance monitoring
pub struct MonitoredAudioBuffer {
    buffer: AudioBuffer,
    stats: Arc<Mutex<BufferStats>>,
}

impl MonitoredAudioBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: AudioBuffer::new(capacity),
            stats: Arc::new(Mutex::new(BufferStats::default())),
        }
    }

    pub fn push(&self, samples: &[f32]) -> usize {
        let written = self.buffer.push(samples);

        let mut stats = self.stats.lock();
        if written < samples.len() {
            stats.overflows += 1;
        }
        stats.total_samples += written as u64;

        let fill_ratio = self.buffer.fill_ratio();
        stats.peak_fill_ratio = stats.peak_fill_ratio.max(fill_ratio);

        written
    }

    pub fn pop(&self, output: &mut [f32]) -> Result<usize> {
        let read = self.buffer.pop(output);

        let mut stats = self.stats.lock();
        if read < output.len() {
            stats.underflows += 1;
            // Only return error if completely empty
            if read == 0 {
                return Err(StreamError::BufferUnderflow);
            }
        }

        Ok(read)
    }

    pub fn stats(&self) -> BufferStats {
        *self.stats.lock()
    }

    pub fn reset_stats(&self) {
        *self.stats.lock() = BufferStats::default();
    }

    pub fn buffer(&self) -> &AudioBuffer {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_push_pop() {
        let buffer = AudioBuffer::new(1024);

        // Push samples
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let written = buffer.push(&input);
        assert_eq!(written, 5);
        assert_eq!(buffer.available(), 5);

        // Pop samples
        let mut output = vec![0.0; 5];
        let read = buffer.pop(&mut output);
        assert_eq!(read, 5);
        assert_eq!(output, input);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_overflow() {
        let buffer = AudioBuffer::new(10);

        let input = vec![1.0; 15];  // More than capacity
        let written = buffer.push(&input);
        assert_eq!(written, 10);  // Only 10 written
        assert!(buffer.is_full());
    }

    #[test]
    fn test_buffer_stats() {
        let buffer = MonitoredAudioBuffer::new(10);

        // Cause overflow
        let input = vec![1.0; 15];
        buffer.push(&input);

        let stats = buffer.stats();
        assert_eq!(stats.overflows, 1);
        assert_eq!(stats.total_samples, 10);
    }

    #[test]
    fn test_fill_ratio() {
        let buffer = AudioBuffer::new(100);

        assert_eq!(buffer.fill_ratio(), 0.0);

        buffer.push(&vec![1.0; 50]);
        assert_eq!(buffer.fill_ratio(), 0.5);

        buffer.push(&vec![1.0; 50]);
        assert_eq!(buffer.fill_ratio(), 1.0);
    }
}
