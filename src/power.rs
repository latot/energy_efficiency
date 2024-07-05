use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, read_dir};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct PowerFile {
    file: OsString,
    data: Vec<usize>,
}

impl PowerFile {
    pub fn new(file: OsString) -> Self {
        PowerFile {
            file: file,
            data: vec![],
        }
    }
    pub fn read(&mut self) -> Result<(), Box<dyn Error>> {
        let value = fs::read_to_string(&self.file)?;
        self.data.push(value.trim().parse::<usize>()?);
        Ok(())
    }
}

pub struct Power {
    files: Arc<Mutex<Vec<PowerFile>>>,
    recorder: Option<thread::JoinHandle<()>>,
    each: Duration,
    alive: Arc<Mutex<bool>>,
}

impl Power {
    pub fn try_new(each: Duration) -> Result<Self, Box<dyn Error>> {
        let mut files: Vec<PowerFile> = vec![];
        for path in read_dir("/sys/class/powercap/")? {
            let path = path?.path();
            if path.is_dir() {
                let file = path.join("energy_uj");
                if file.exists() && file.is_file() {
                    files.push(PowerFile::new(file.into()));
                }
            }
        }
        Ok(Power {
            files: Arc::new(Mutex::new(files)),
            recorder: None,
            each: each,
            alive: Arc::new(Mutex::new(false)),
        })
    }
    pub fn read(&mut self) -> Result<(), Box<dyn Error>> {
        for file in self.files.clone().lock().unwrap().iter_mut() {
            file.read()?;
        }
        Ok(())
    }

    pub fn start(&mut self) {
        self.stop();
        let files = self.files.clone();
        let each = self.each.clone();
        {
            let arc_alive = self.alive.clone();
            let mut alive = arc_alive.lock().unwrap();
            *alive = true;
        }
        let alive = self.alive.clone();
        self.recorder = Some(thread::spawn(move || {
            let mut files = files.lock().unwrap();
            let alive = alive;
            loop {
                for file in files.iter_mut() {
                    file.read().unwrap();
                }
                thread::sleep(each);
                let alive = alive.lock().unwrap();
                println!("Checking alive {}", *alive);
                if !*alive {
                    break;
                }
            }
            println!("internal stop");
        }));
    }

    pub fn stop(&mut self) {
        let arc_alive = self.alive.clone();
        let mut mutex_alive = arc_alive.lock().unwrap();
        if *mutex_alive {
            *mutex_alive = false;
            drop(mutex_alive);
            drop(arc_alive);
            println!("Stopping");
            while !self.recorder.as_ref().unwrap().is_finished() {
                thread::sleep(Duration::from_micros(100));
            }
            self.recorder = None;
        }
    }

    pub fn save(&self, path: OsString) {
        todo!()
    }

    pub fn print(&self) {
        let arc_files = self.files.clone();
        let files = arc_files.lock().unwrap();
        for file in files.iter() {
            println!("Data {:?}", file.data);
            println!("File {}", file.file.clone().into_string().unwrap());
        }
    }
}
