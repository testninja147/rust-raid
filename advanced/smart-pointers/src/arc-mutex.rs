use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
/**
 * # Mutex
 * Mutex ( MUTual EXclusion ) is a synchronization primitive that allows multiple
 * threads to access a shared resource safely.
 *
 * Think mutex as a rented bike which can be used by multiple users, however,
 * one should first reserve before being able to ride. The second user will be
 * able to ride the bike only once the first user returns it to the owner.
 *
 * In case of mutex, a thread must lock the resource to be able to mutate it.
 * after its operation is completed, the thread releases the lock and the next
 * thread in the queue will be able to use it.
 *
 * when we lock a mutex, it returns a smart pointer to the resource called a
 * MutexGuard which is a smart pointer that releases the lock when it goes out
 * of scope.
 */

struct Rent {
    user: String,
    time: usize,
}

struct Bike {
    ride_time: usize,
}

impl Bike {
    fn new() -> Self {
        Self { ride_time: 0 }
    }

    fn ride(&mut self, time: usize) {
        println!("A {} hours bike ride has been started", time);
        thread::sleep(Duration::from_secs(time as u64));
        self.ride_time += time;
        println!("ride complete!!");
    }
}

impl Rent {
    fn new(user: &str, time: usize) -> Self {
        Self {
            user: user.to_owned(),
            time,
        }
    }
}

fn main() {
    // in this example, a bike will be rented by 4 different people, however
    // it is not possible to ride a bike by all at once, so mutex guard will
    // make sure that the bike is mutated by only one user.
    //
    println!("\n{}", "=".repeat(50));

    let bike = Arc::new(Mutex::new(Bike::new()));
    let rentals = vec![
        Rent::new("Alice", 5),
        Rent::new("Bob", 2),
        Rent::new("Charlie", 4),
        Rent::new("Daniel", 3),
    ];

    // we keep track of thread handles here
    let mut handles = vec![];

    // iterate through rentals
    for rent in rentals {
        // clone for each thread
        let bike = Arc::clone(&bike);

        // create a thread handle
        // here we need to have the ownership of the bike to be able to mutate
        // so we need to use move along with closure
        let handle = thread::spawn(move || {
            // since the process is threaded, waiting order might
            // differ on each execution.
            println!("[{:^20}] waiting!!", &rent.user.clone());

            let mut rental = bike.lock().unwrap();
            println!("{}", "-".repeat(50));
            println!(
                "The bike is now rented by {} for {} hours.",
                &rent.user.clone(),
                &rent.time
            );

            // the ride() method emulated bike ride and makes the thread sleep
            // for the given time in seconds. Until the thread is completed,
            // another thread will not be able to lock the bike so it will wait
            // until previous thread is released.

            // If you comment the line below, there will not be any mutation so
            // the order of execution may be different so output will be jumbled.
            rental.ride(rent.time);

            println!("The bike ran {} hours.", rental.ride_time);
            println!("[ The bike has been returned ]")
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "The bike had a total ride of {} hours",
        bike.as_ref().lock().unwrap().ride_time
    );
}
