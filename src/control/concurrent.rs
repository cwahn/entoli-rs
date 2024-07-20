use crate::prelude::Io;

#[derive(Clone)]
pub struct RecIo<I> {
    inner: I,
}

impl<I> Io for RecIo<I>
where
    I: Io<Output = ()> + Clone,
{
    type Output = ();

    fn run(self) -> Self::Output {
        loop {
            self.inner.clone().run()
        }
    }
}

pub fn rec<I>(io: I) -> RecIo<I>
where
    I: Io<Output = ()> + Clone,
{
    RecIo { inner: io }
}

#[derive(Clone)]
pub struct DelayForIo {
    duration: std::time::Duration,
}

impl Io for DelayForIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::thread::sleep(self.duration);
    }
}

pub fn delay_for(duration: std::time::Duration) -> DelayForIo {
    DelayForIo { duration }
}

#[derive(Clone)]
pub struct DelayUntilIo {
    instant: std::time::Instant,
}

impl Io for DelayUntilIo {
    type Output = ();

    fn run(self) -> Self::Output {
        let now = std::time::Instant::now();
        if now < self.instant {
            std::thread::sleep(self.instant - now);
        }
    }
}

pub fn delay_until(instant: std::time::Instant) -> DelayUntilIo {
    DelayUntilIo { instant }
}
