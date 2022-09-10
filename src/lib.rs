use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; 

impl ThreadPool {
    // usize - pointer-sized UNSIGNED integer type
    //         allows you to reference any location in memory 
    //         and bc of that - isn't a set, fixed size of bytes - bc
    //         it takes on a different size in bytes depending on how many 
    //         bytes it takes to reference a particularly sized piece of 
    //         data in memory
    ///  Create a new ThreadPool. 
    ///
    ///  The size is the number of threads in the pool 
    ///
    ///  # Panics 
    ///
    ///  The `new` function will panic if the size is zero
    pub fn new(size: usize) ->  ThreadPool {
        assert!(size > 0);

        // - allows vector in memory to hold elemnts until CAPACITY without 
        //   the vector needing to reallocated (reserves future real estate 
        //   for the vector)
        // - preallocation is more efficient for performance than allocating
        //   on a need-to basis
        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // .push implictly takes a mutable reference to the value coming 
            // before "."
            workers.push(Worker::new(id, Arc::clone(&receiver)));
            // create some threads & store them in vector
        }
    
        ThreadPool { workers, sender: Some(sender) }
    }

    ///  Executes a ThreadPool. 
    ///
    ///  f is a closure that implements traits FnOnce(), Send, and 'static
    ///
    ///  # Panics 
    ///
    ///  The `execute` function will panic if a transmitter fails to send 
    ///  a job to a receiver
    pub fn execute<F>(&self, f: F)
        where 
        // "()" - denotes that closure (a closure that returns a unit type
        //        since there is '->' showing signature of F)
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop (self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // could you delete this check, so you always join the 
            //       thread contained insdie Option w out checking 
            //       whether or not
            //       Option is Some or None? 
            if let Some(thread) = worker.thread.take() {
                // "take" takes Some variant out, and leaves behind a None 
                //        variant 
                // * "take" doesn't unwrap the value, it takes the ENTIRE 
                //        Some variant
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    // JoinHandle<T>'s T is the type our closure returns 
    //             since closure only runs statement, handle_connection
    //             , we can use a unit type for the return value of our 
    //             closure
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // w "let" - any temporary values used in expression on
        //           RH side of equals above is immediately 
        //           dropped when "let" statement ends (i.e., when 
        //           line 78 is reached)
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}





