use std::thread;

pub trait OffThreadExt: Iterator {
    /// Transform this iterator into an off-thread iterator: the
    /// `next()` call happen on a separate worker thread, so the
    /// iterator and the body of the loop run concurrently.
    fn off_thread(self) -> crossbeam::channel::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> crossbeam::channel::IntoIter<Self::Item> {
        // Create a channel to transfer items from the worker thread
        let (sender, receiver) = crossbeam::channel::bounded(1024);

        // Move tis iterator to a new worker thread and run it there
        thread::spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });

        // Return an iterator that pulls values from channels
        receiver.into_iter()
    }
}
