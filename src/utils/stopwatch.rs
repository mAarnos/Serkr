// Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.
//
// Serkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Serkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Serkr. If not, see <http://www.gnu.org/licenses/>.
//

extern crate time;

/// A stopwatch-type timer.
#[derive(Debug, Copy, Clone)]
pub struct Stopwatch {
    start_time: u64,
    stop_time: u64,
    running: bool,
}

impl Stopwatch {
    /// Creates a new stopwatch.
    pub fn new() -> Stopwatch {
        Stopwatch {
            start_time: 0,
            stop_time: 0,
            running: false,
        }
    }

    /// Starts the stopwatch.
    pub fn start(&mut self) {
        self.start_time = time::precise_time_ns();
        self.running = true;
    }

    /// Stops the stopwatch.
    pub fn stop(&mut self) {
        self.stop_time = time::precise_time_ns();
        self.running = false;
    }

    /// Resets the stopwatch.
    pub fn reset(&mut self) {
        self.start_time = 0;
        self.stop_time = 0;
        self.running = false;
    }

    /// Checks if the stopwatch is currently running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Returns the elapsed time between the starting and either the current or stopping time.
    /// Obviously the time will be in milliseconds.
    pub fn elapsed_ms(&self) -> u64 {
        (if self.running {
            time::precise_time_ns()
        } else {
            self.stop_time
        } - self.start_time) / 1000000
    }
}

impl Default for Stopwatch {
    fn default() -> Stopwatch {
        Stopwatch::new()
    }
}
