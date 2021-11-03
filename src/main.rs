use std::{thread, time};
use std::sync::mpsc;

use rand::thread_rng;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum MachineStates{
    Destitute,
    Working,
    Idle,
}


#[derive(Debug)]
pub struct Machine {
    state: MachineStates,
    pub cond_a: bool,
    pub cond_b: bool,
}

impl Machine{

    pub fn new() -> Machine{
        Machine{
            state: MachineStates::Idle,
            cond_a: false,
            cond_b: false,
        }
    }

    fn get_next_state(&self) -> MachineStates{
        match self.state {
            MachineStates::Destitute =>{
                if self.cond_a && self.cond_b{
                    return MachineStates::Working;
                }
                return MachineStates::Destitute;
            }
            MachineStates::Working => {
                return MachineStates::Idle;
            },
            MachineStates::Idle => {
                return MachineStates::Destitute;
            },
        };
    }

    pub fn machine(&mut self){
        let next_state = self.get_next_state();

        if self.state != next_state{
            println!("New state = {:?} ", next_state);
        }

        self.state = next_state;
        match self.state {
            MachineStates::Destitute =>{
                println!("DESTITUDE and waiting for conditions");
            },
            MachineStates::Working =>{
                self.cond_a = false;
                self.cond_b = false;

                let dur = time::Duration::from_millis(2000);
                thread::sleep(dur);
                println!("WORKING Completed Sleep");
            }
            MachineStates::Idle =>{
                println!("IDLE next is DESTITIUE");

            }
        }
    }
}

fn condition_a(chan: mpsc::Sender<String>) {
    let mut rng = thread_rng();

    loop{ 
        let sec: f32 = rng.gen_range(0.5, 10.0);
        let dur = time::Duration::from_secs_f32(sec);
        println!("Condition A sleep={:?}",dur);
        thread::sleep(dur);
        match chan.send(format!("A")){
            Ok(_) => (),
            Err(e) => println!("Error With Condition A send {:?}", e),
        };
    }
}

fn condition_b(chan: mpsc::Sender<String>) {
    let mut rng = thread_rng();

    loop{
        let sec: f32 = rng.gen_range(0.5, 10.0);
        let dur = time::Duration::from_secs_f32(sec);
        println!("Condition B sleep={:?}",dur);
        thread::sleep(dur);
        match chan.send(format!("B")){
            Ok(_) => (),
            Err(e) => println!("Error With Condition B send {:?}", e),
        };
    }
}

fn main() {

    let mut machine = Box::new(Machine::new());

    let (tx, mut rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();

    thread::spawn(|| {
        condition_a(tx);
    });

    thread::spawn(move || {
        condition_b(tx2);
    });

    loop{

        while let Ok(message) = rx.recv() {
            println!("Recieved Condition = {}", message);
            match message.as_str(){
                "B" => machine.cond_a = true,
                "A" => machine.cond_b = true,
                _ => println!("idk where that came from...")
            };
            
            //Call State Machine with new condition
            machine.machine();
        }
    }
}