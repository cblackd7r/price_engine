extern crate cronjob;
#[macro_use]
extern crate lazy_static;

use cronjob::CronJob;
use chrono::{Timelike, Utc};
use std::thread;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

static GLOBAL_PRICE_ZERO_INPUT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_PRICE_ONE_INPUT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_PRICE_TWO_INPUT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_METRIC_ONE_INPUT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_METRIC_TWO_INPUT: AtomicUsize = AtomicUsize::new(0); //Zero (false), One (true)

const RUNNING_EMOJI: char = '\u{23F3}';
const ALARM_EMOJI: char = '\u{23F0}';
const ROCKET_EMOJI: char = '\u{1F680}';
const OFF_EMOJI: char = '\u{1F534}';
const ON_EMOJI: char = '\u{1F7E2}';
const LIMITED_EMOJI: char = '\u{1F7E8}';
const CHECK_EMOJI: char = '\u{2705}';


lazy_static! {
    static ref ARRAY: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub fn main() {

    let args: Vec<String> = env::args().collect();

    let price_zero_input = &args[1];
    let price_zero = price_zero_input.parse::<u32>().unwrap();
    
    let price_one_input = &args[2];
    let price_one = price_one_input.parse::<u32>().unwrap();
    
    let price_two_input = &args[3];
    let price_two = price_two_input.parse::<u32>().unwrap();
        
    let metric_one_input = &args[4];
    let metric_one = metric_one_input.parse::<u32>().unwrap();
    
    let metric_two_input = &args[5];
    let bool_metric_two = metric_two_input.parse::<bool>().unwrap();
   
    
    run(price_zero, price_one, price_two, metric_one, bool_metric_two);
  
    
}
pub fn run(price_zero_input: u32, price_one_input: u32, price_two_input: u32, metric_one_input: u32, metric_two_input: bool) {

    match price_zero_input {
        1..=200 if price_zero_input < 200 => (),
        200 => println!("Limit: {}", price_zero_input),
        _ => { 
            panic!("Value for price_zero_input is out of range (should be 1 to 200) ");
        }
    }

    match price_one_input {
        1..=200 if price_one_input < 200 => (),
        200 => println!("Limit: {}", price_zero_input),
        _ => { 
            panic!("Value for price_one_input is out of range (should be 1 to 200) "); 
        }
    }

    match price_two_input {
        1..=10 if price_two_input < 10 => (),
        10 => println!("Limit: {}", price_two_input),
        _ => { 
            panic!("Value for price_two_input is out of range (should be 1 to 10) ");
        }
    }

    match metric_one_input {
        1..=10 if price_zero_input < 10 => (),
        200 => println!("Limit: {}", metric_one_input),
        _ => { 

            panic!("Value for metric_one_input is out of range (should be 1 to 10) ");
        }
    }


    let mut binary_metric_two_input: u32 = 0;

    if metric_two_input{
        binary_metric_two_input = 1;
    }

    
    GLOBAL_PRICE_ZERO_INPUT.fetch_add(price_zero_input.try_into().unwrap(), Ordering::SeqCst);
    GLOBAL_PRICE_ONE_INPUT.fetch_add(price_one_input.try_into().unwrap(), Ordering::SeqCst);
    GLOBAL_PRICE_TWO_INPUT.fetch_add(price_two_input.try_into().unwrap(), Ordering::SeqCst);
    GLOBAL_METRIC_ONE_INPUT.fetch_add(metric_one_input.try_into().unwrap(), Ordering::SeqCst);
    GLOBAL_METRIC_TWO_INPUT.fetch_add(binary_metric_two_input.try_into().unwrap(), Ordering::SeqCst);
    
    // Create the `CronJob` objects for price_zero_input input.
    let mut price_zero_input_cron = CronJob::new("price_zero_input", price_zero_input_on_cron);
    // Set to fire when seconds is 0, 2 or 4
    price_zero_input_cron.seconds("15");
    // Set offset for UTC
    price_zero_input_cron.offset(0);

    // Create the `CronJob` objects for price_one_input input.
    let mut price_one_input_cron = CronJob::new("price_one_input", price_one_input_on_cron);
    // Set to fire when seconds is 0, 2 or 4
    price_one_input_cron.seconds("5");
    // Set offset for UTC
    price_one_input_cron.offset(0);
    
    // Create the `CronJob` objects for price_two_input input.
    let mut price_two_input_cron = CronJob::new("price_two_input", price_two_input_on_cron);
    // Set to fire when seconds is 0, 2 or 4
    price_two_input_cron.seconds("7");
    // Set offset for UTC
    price_two_input_cron.offset(0);

    // Create the `CronJob` objects for metric_one_input input.
    let mut metric_one_input_cron = CronJob::new("metric_one_input", metric_one_input_on_cron);
    // Set to fire when seconds is 0, 2 or 4
    metric_one_input_cron.seconds("8");
    // Set offset for UTC
    metric_one_input_cron.offset(0);

    // Create the `CronJob` objects for 4CP input.
    let mut metric_two_input_cron = CronJob::new("metric_two_input", metric_two_input_on_cron);
    // Set to fire when seconds is 0, 2 or 4
    metric_two_input_cron.seconds("0");
    // Set offset for UTC
    metric_two_input_cron.offset(0);

    // Create the `CronJob` objects for collector control loop input.
    let mut collect_contol_cron = CronJob::new("metric_two_input", collect_control_on_cron);
    // Set to fire when seconds is 0, 2 or 4
    collect_contol_cron.seconds("0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59");
    // Set offset for UTC
    metric_two_input_cron.offset(0);
   
    let price_zero_input_sched_thread = thread::spawn(move || {
        price_zero_input_cron.start_job();
    });

    let price_one_input_sched_thread = thread::spawn(move || {
        price_one_input_cron.start_job();
    });

    let price_two_input_sched_thread = thread::spawn(move || {
        price_two_input_cron.start_job();
    });

    let metric_one_input_sched_thread = thread::spawn(move || {
        metric_one_input_cron.start_job();
    });

    let metric_two_input_sched_thread = thread::spawn(move || {
        metric_two_input_cron.start_job();
    });

    let collect_contol_sched_thread = thread::spawn(move || {
        collect_contol_cron.start_job();
    });


    let run_price_zero_input_sched_thread = price_zero_input_sched_thread.join();
    match run_price_zero_input_sched_thread {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

    let run_price_one_input_sched_thread  = price_one_input_sched_thread.join();
    match run_price_one_input_sched_thread {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

    let run_price_two_input_sched_thread  = price_two_input_sched_thread.join();
    match run_price_two_input_sched_thread {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

    let run_metric_one_input_sched_thread  = metric_one_input_sched_thread.join();
    match run_metric_one_input_sched_thread {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

    let run_metric_two_input_sched_thread  = metric_two_input_sched_thread.join();
    match run_metric_two_input_sched_thread {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

    let run_collect_contol_sched_thread = collect_contol_sched_thread.join();
    match run_collect_contol_sched_thread{
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_range_600() {
        run(600, 600, 600, 600, false);
    }
}

// price_zero_input cronjob handler
fn price_zero_input_on_cron(name: &str) {
    println!("It's time!  {} {}", name, ALARM_EMOJI);
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    ARRAY.lock().unwrap().push(name.to_string());
}

// price_one_input cronjob handler
fn price_one_input_on_cron(name: &str) {
    println!("It's time!  {} {}", name, ALARM_EMOJI);
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    ARRAY.lock().unwrap().push(name.to_string());
}

// price_two_input cronjob handler
fn price_two_input_on_cron(name: &str) {
    println!("It's time!  {} {}", name, ALARM_EMOJI);
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    ARRAY.lock().unwrap().push(name.to_string());
}

// metric_one_input cronjob handler
fn metric_one_input_on_cron(name: &str) {
    println!("It's time!  {} {}", name, ALARM_EMOJI);
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    ARRAY.lock().unwrap().push(name.to_string());
}

// 4CP cronjob handler
fn metric_two_input_on_cron(name: &str) {
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!("It's time!  {} {}", name, ALARM_EMOJI);
    println!(
        "UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    ARRAY.lock().unwrap().push(name.to_string());
}

fn collect_control_on_cron(_name: &str) {
    let mut price_confirm: bool = false;
    let mut price_one_input_confirm: bool = false;
    let mut price_two_input_confirm: bool = false;
    let mut metric_one_input_confirm: bool = false;
    let mut metric_two_input_confirm: bool = false;

    println!("Checking collection completion state...  {}", RUNNING_EMOJI);
    let _vec: Vec<String> = Vec::new();
    let vec = ARRAY.lock().unwrap().clone();
    if vec.len() > 4 {   
        for e in vec {
            println!("{}", e);
            if e.contains("price_zero_input") {
                price_confirm = true;
            }
            if e.contains("price_one_input") {
                price_one_input_confirm = true;
            }
            if e.contains("price_two_input") {
                price_two_input_confirm = true;
            }
            if e.contains("metric_one_input") {
                metric_one_input_confirm = true;
            }
            if e.contains("metric_two_input") {
                metric_two_input_confirm = true;
            }

            if price_confirm && price_one_input_confirm && price_two_input_confirm && metric_one_input_confirm && metric_two_input_confirm {
                println!("{} {}", "All data collected! ", CHECK_EMOJI);
                start_control_state();         
            }

        }
    }
}

fn start_control_state() {

    println!("Control state started. {}", ROCKET_EMOJI);

    let in_state_off = StateMachine::new();

    let local_price_zero  = GLOBAL_PRICE_ZERO_INPUT.load(Ordering::SeqCst);
    let price_zero = local_price_zero  as u32;

    let local_price_one  = GLOBAL_PRICE_ONE_INPUT.load(Ordering::SeqCst);
    let price_one = local_price_one  as u32;

    let local_price_two = GLOBAL_PRICE_TWO_INPUT.load(Ordering::SeqCst);
    let price_two= local_price_two as u32;

    let local_metric_one  = GLOBAL_METRIC_ONE_INPUT.load(Ordering::SeqCst);
    let metric_one = local_metric_one  as u32;

    let local_metric_two = GLOBAL_METRIC_TWO_INPUT.load(Ordering::SeqCst);
    let metric_two= local_metric_two as u32;

    let mut bool_metric_two: bool = false;
    if metric_two== 1 {
        bool_metric_two= true;
    }

    //start control logic

    if bool_metric_two {
        if price_one < 1 {
            let _in_state_limited = StateMachine::<StateOFF>::from(in_state_off);
            println!("Finished in Limited On state {}", LIMITED_EMOJI);
        } else {
            println!("Stays in Off state {}", OFF_EMOJI);
        }  
    } else if price_two> 0 {
        if price_one > 1 {
            if price_two> metric_one {
                let _in_state_on = StateMachine::<StateON>::from(in_state_off);
                println!("On state {}", ON_EMOJI);
            } else if price_two< metric_one {
                    let _in_state_off_from_blend_from_price_two_less_metric_one_input = StateMachine::new();
                    let _in_state_limited = StateMachine::<StateLIMITED>::from(_in_state_off_from_blend_from_price_two_less_metric_one_input);
                    println!("On state {}", ON_EMOJI);
            } else {
                let _in_state_off_from_blend_from_price_two_less_metric_one_input_greater_10= StateMachine::new();
                let _in_state_on = StateMachine::<StateON>::from(_in_state_off_from_blend_from_price_two_less_metric_one_input_greater_10);
                println!("On state {}", ON_EMOJI);
            }
            
            let blended_price = price_zero + price_one * price_two / 10;
            println!("Blended Price:  {}", blended_price);

            if blended_price > 10 {
                println!("{} {}", "Finished in Off state", OFF_EMOJI);
            } else {
                let _in_state_off_from_blend = StateMachine::new();
                let _in_state_on = StateMachine::<StateON>::from(_in_state_off_from_blend);
                println!("Finished in On state {}", LIMITED_EMOJI);
            }
        } else {
            println!("Finished in Off state {}", OFF_EMOJI);
        }
    } else if  price_zero > 10 {
            println!("Finished in Off state {}", OFF_EMOJI);
        } else {
            let _in_state_on = StateMachine::<StateON>::from(in_state_off);
            println!("Finished in On state {}", ON_EMOJI);
        }
    
}


struct StateMachine<S> {
    _state: S,
}


// It starts, predictably, in `OFF`
impl StateMachine<StateOFF> {
    fn new() -> Self {
        StateMachine {
            _state: StateOFF::new()
        }
    }
}

// State OFF starts the machine in OFF state
struct StateOFF {
    _start_value: String,
}


impl StateOFF {
    fn new() -> Self {
        StateOFF {
            _start_value: "OFF".to_string(),
        }
    }
}



// State LIMITED goes as LIMITED ON
struct StateLIMITED {
    _limited_value: String,
}

impl From<StateMachine<StateOFF>> for StateMachine<StateLIMITED> {
    fn from(_val: StateMachine<StateOFF>) -> StateMachine<StateLIMITED> {
        StateMachine {
            _state: StateLIMITED {
                _limited_value: "LIMITED".to_string(),
            }
        }
    }
}

// Finally, State ON gives us ON state
struct StateON {
    _final_value: String,
}
impl From<StateMachine<StateLIMITED>> for StateMachine<StateON> {
    fn from(_val: StateMachine<StateLIMITED>) -> StateMachine<StateON> {
        StateMachine {
            _state: StateON {
                _final_value: "ON".to_string(),
            }
        }
    }
}

impl From<StateMachine<StateOFF>> for StateMachine<StateON> {
    fn from(_val: StateMachine<StateOFF>) -> StateMachine<StateON> {
        StateMachine {
            _state: StateON {
                _final_value: "ON".to_string(),
            }
        }
    }
}

