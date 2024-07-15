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
    delay: std::time::Duration,
}

impl Io for DelayForIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::thread::sleep(self.delay);
    }
}

pub fn delay_for(delay: std::time::Duration) -> DelayForIo {
    DelayForIo { delay }
}

#[derive(Clone)]
pub struct DelayUntilIo {
    deadline: std::time::Instant,
}

impl Io for DelayUntilIo {
    type Output = ();

    fn run(self) -> Self::Output {
        let now = std::time::Instant::now();
        if now < self.deadline {
            std::thread::sleep(self.deadline - now);
        }
    }
}

pub fn delay_until(deadline: std::time::Instant) -> DelayUntilIo {
    DelayUntilIo { deadline }
}
