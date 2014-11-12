extern crate sdl2;



pub struct Time {
    last_frame_ticks: u64,
    since_start_ns: u64,
    last_frame_ns: u64,
    performance_counter: u64,
    performance_frequency: u64,
    seconds_timer_ns: u64,
    second_flag: bool,
}

impl Time {
    pub fn new() -> Time {
        Time {
            last_frame_ticks: sdl2::timer::get_performance_counter(),
            since_start_ns: 0,
            last_frame_ns: 0,
            performance_counter: 0,
            performance_frequency: 0,
            seconds_timer_ns: 0,
            second_flag: false,
        }
    }

    pub fn update(&mut self) {
        self.performance_counter = sdl2::timer::get_performance_counter();
        self.performance_frequency = sdl2::timer::get_performance_frequency();

        //println!("{} {}", self.performance_counter, self.performance_frequency);

        let diff = self.performance_counter - self.last_frame_ticks;
        self.last_frame_ticks = self.performance_counter;

        let ns = diff * 1_000_000_000_u64 / self.performance_frequency;

        self.last_frame_ns = self.since_start_ns;
        self.since_start_ns += ns;

        self.seconds_timer_ns += ns;
        if (self.seconds_timer_ns >= 1_000_000_000_u64) {
            self.seconds_timer_ns = 0;
            self.second_flag = true;
        }
    }

    pub fn delta(&self) -> f32 {
        (self.since_start_ns as f32 - self.last_frame_ns as f32) / 1_000_000_000_f32
    }

    pub fn second_elapsed(&mut self) -> bool {
        if (self.second_flag) {
            self.second_flag = false;
            return true;
        }
        return false;
    }
}