use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// A Sliding Window Rate Limiter using Rust Collections
///
/// It uses a `HashMap` to keep track of different clients (e.g., users or IP addresses).
/// For each client, it uses a `VecDeque` (a double-ended queue) to store the timestamps
/// of their recent requests.
struct RateLimiter {
    clients: HashMap<String, VecDeque<Instant>>,
    window_size: Duration,
    max_requests: usize,
}

impl RateLimiter {
    /// Creates a new RateLimiter
    fn new(window_size: Duration, max_requests: usize) -> Self {
        RateLimiter {
            clients: HashMap::new(),
            window_size,
            max_requests,
        }
    }

    /// Checks if a request from a specific client should be allowed or denied
    fn check_request(&mut self, client_id: &str) -> bool {
        let now = Instant::now();
        // Calculate the cutoff time for the sliding window
        let window_start = now.checked_sub(self.window_size).unwrap_or(now);

        // Get the queue of timestamps for this client, or create a new one if it doesn't exist
        let timestamps = self
            .clients
            .entry(client_id.to_string())
            .or_insert_with(VecDeque::new);

        // 1. Remove outdated timestamps from the front of the queue
        while let Some(&oldest) = timestamps.front() {
            if oldest < window_start {
                timestamps.pop_front();
            } else {
                break;
            }
        }

        // 2. Check if the client has exceeded the limit
        if timestamps.len() < self.max_requests {
            timestamps.push_back(now);
            true
        } else {
            false
        }
    }
}

fn main() {
    // Configure rate limiter: Allow a maximum of 5 requests per 2 seconds
    let mut limiter = RateLimiter::new(Duration::from_secs(2), 5);

    let client_a = "user_123";
    let client_b = "user_456";

    println!("--- Sliding Window Rate Limiter Simulation ---");
    println!("Configuration: Max 5 requests per 2 seconds\n");

    println!("Simulating rapid requests for Client A...");

    // Send 7 rapid requests for client A
    for i in 1..=7 {
        if limiter.check_request(client_a) {
            println!("[✓]: Client A - Request {}: Allowed", i);
        } else {
            println!(
                "[✗]: Client A - Request {}: DENIED (Rate limit exceeded)",
                i
            );
        }
        // Small delay to simulate time passing between requests
        std::thread::sleep(Duration::from_millis(50));
    }

    println!("\nSimulating a request for Client B...");
    if limiter.check_request(client_b) {
        println!("[✓]: Client B - Request 1: Allowed");
    }

    println!("\nWaiting for 2 seconds to let the time window slide...");
    std::thread::sleep(Duration::from_secs(2));

    println!("\nClient A sends requests after waiting:");
    for i in 8..=10 {
        if limiter.check_request(client_a) {
            println!("Client A - Request {}: Allowed ✅", i);
        } else {
            println!("Client A - Request {}: DENIED ❌", i);
        }
    }
}
