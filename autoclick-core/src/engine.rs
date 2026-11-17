use crate::config::{AppConfig, ClickType, PositionMode, RepeatMode};
use crate::input::InputBackend;
use rand::Rng;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
pub struct EngineStatus {
    pub running: bool,
    pub clicks: u64,
    pub elapsed_ms: u64,
}

pub fn compute_delay_ms(
    base_ms: u64,
    jitter_enabled: bool,
    jitter_ms: u64,
    rng: &mut impl Rng,
) -> u64 {
    let base = base_ms.max(1);
    if !jitter_enabled || jitter_ms == 0 {
        return base;
    }
    let j = jitter_ms as i64;
    let offset = rng.gen_range(-j..=j);
    let result = (base as i64).saturating_add(offset);
    result.max(1) as u64
}

pub struct ClickEngine {
    running: Arc<AtomicBool>,
    clicks: Arc<AtomicU64>,
    elapsed_ms: Arc<AtomicU64>,
    backend: Arc<dyn InputBackend>,
}

impl ClickEngine {
    pub fn new(backend: Arc<dyn InputBackend>) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            clicks: Arc::new(AtomicU64::new(0)),
            elapsed_ms: Arc::new(AtomicU64::new(0)),
            backend,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn status(&self) -> EngineStatus {
        EngineStatus {
            running: self.running.load(Ordering::SeqCst),
            clicks: self.clicks.load(Ordering::SeqCst),
            elapsed_ms: self.elapsed_ms.load(Ordering::SeqCst),
        }
    }

    pub fn stop(&self) -> bool {
        self.running.swap(false, Ordering::SeqCst)
    }

    pub fn start(
        &self,
        config: AppConfig,
        on_status: Arc<dyn Fn(EngineStatus) + Send + Sync>,
    ) -> bool {
        if self
            .running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return false;
        }

        self.clicks.store(0, Ordering::SeqCst);
        self.elapsed_ms.store(0, Ordering::SeqCst);

        let running = Arc::clone(&self.running);
        let clicks = Arc::clone(&self.clicks);
        let elapsed_ms = Arc::clone(&self.elapsed_ms);
        let backend = Arc::clone(&self.backend);

        std::thread::spawn(move || {
            let start_time = Instant::now();
            let mut last_emit = Instant::now();
            let mut rng = rand::thread_rng();

            let pos = match config.position_mode {
                PositionMode::Current => None,
                PositionMode::Fixed => Some((config.fixed_x, config.fixed_y)),
            };

            on_status(EngineStatus {
                running: true,
                clicks: 0,
                elapsed_ms: 0,
            });

            if config.click_type == ClickType::Hold {
                backend.mouse_down(config.button, pos);
                clicks.store(1, Ordering::SeqCst);
                while running.load(Ordering::SeqCst) {
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    elapsed_ms.store(elapsed, Ordering::SeqCst);
                    if last_emit.elapsed() >= Duration::from_millis(80) {
                        on_status(EngineStatus {
                            running: true,
                            clicks: 1,
                            elapsed_ms: elapsed,
                        });
                        last_emit = Instant::now();
                    }
                    std::thread::sleep(Duration::from_millis(20));
                }
                backend.mouse_up(config.button);
            } else {
                let mut count: u64 = 0;
                while running.load(Ordering::SeqCst) {
                    backend.click(config.button, config.click_type, pos);
                    count += 1;
                    clicks.store(count, Ordering::SeqCst);
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    elapsed_ms.store(elapsed, Ordering::SeqCst);

                    if last_emit.elapsed() >= Duration::from_millis(50) {
                        on_status(EngineStatus {
                            running: true,
                            clicks: count,
                            elapsed_ms: elapsed,
                        });
                        last_emit = Instant::now();
                    }

                    if config.repeat_mode == RepeatMode::Count && count >= config.repeat_count.max(1)
                    {
                        break;
                    }

                    let delay = compute_delay_ms(
                        config.interval_ms,
                        config.jitter_enabled,
                        config.jitter_ms,
                        &mut rng,
                    );
                    sleep_interruptible(&running, Duration::from_millis(delay));
                }
            }

            running.store(false, Ordering::SeqCst);
            let final_elapsed = start_time.elapsed().as_millis() as u64;
            elapsed_ms.store(final_elapsed, Ordering::SeqCst);
            on_status(EngineStatus {
                running: false,
                clicks: clicks.load(Ordering::SeqCst),
                elapsed_ms: final_elapsed,
            });
        });

        true
    }
}

fn sleep_interruptible(running: &AtomicBool, total: Duration) {
    let chunk = Duration::from_millis(20);
    let mut remaining = total;
    while remaining > Duration::ZERO && running.load(Ordering::SeqCst) {
        let step = remaining.min(chunk);
        std::thread::sleep(step);
        remaining = remaining.saturating_sub(step);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::MockInputBackend;
    use rand::SeedableRng;

    #[test]
    fn delay_clamped_to_minimum_1ms() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let d = compute_delay_ms(5, true, 50, &mut rng);
            assert!(d >= 1);
            assert!(d <= 55);
        }
        assert_eq!(compute_delay_ms(0, false, 0, &mut rng), 1);
    }

    #[test]
    fn engine_stops_after_repeat_count() {
        let backend = Arc::new(MockInputBackend::new());
        let engine = ClickEngine::new(backend.clone());

        let mut cfg = AppConfig::default();
        cfg.interval_ms = 2;
        cfg.repeat_mode = RepeatMode::Count;
        cfg.repeat_count = 5;

        let started = engine.start(cfg, Arc::new(|_| {}));
        assert!(started);

        for _ in 0..50 {
            if !engine.is_running() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }

        assert!(!engine.is_running());
        assert_eq!(backend.click_count.load(Ordering::SeqCst), 5);
        assert_eq!(engine.status().clicks, 5);
    }

    #[test]
    fn engine_hold_mode_presses_and_releases() {
        let backend = Arc::new(MockInputBackend::new());
        let engine = ClickEngine::new(backend.clone());

        let mut cfg = AppConfig::default();
        cfg.click_type = ClickType::Hold;

        assert!(engine.start(cfg, Arc::new(|_| {})));
        std::thread::sleep(Duration::from_millis(40));
        assert!(engine.is_running());
        assert!(engine.stop());

        for _ in 0..20 {
            if backend.up_count.load(Ordering::SeqCst) > 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(backend.down_count.load(Ordering::SeqCst), 1);
        assert_eq!(backend.up_count.load(Ordering::SeqCst), 1);
    }
}
