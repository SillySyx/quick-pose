use std::fs::read_dir;
use core::fmt::Debug;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::settings::Settings;

#[derive(Debug, Clone)]
pub struct Session {
    pub images: Vec<String>,
    pub current_image: usize,
    pub session_time: usize,
    pub pause_time: usize,
}

impl Session {
    pub fn new() -> Self {
        Self {
            images: vec![],
            current_image: 0,
            session_time: 0,
            pause_time: 0,
        }
    }

    pub fn from(settings: &Settings) -> Self {
        let images = read_images_in_folder(&settings.folder);
        let images = select_images(images, settings.images_number);

        let duration = match settings.duration {
            Some(value) => value.as_secs() as usize,
            None => 0,
        };

        let pause = match settings.pause {
            Some(value) => value.as_secs() as usize,
            None => 0,
        };

        Self {
            current_image: 0,
            images,
            session_time: duration,
            pause_time: pause,
        }
    }

    pub fn current_image(&self) -> Option<String> {
        match self.images.get(self.current_image) {
            Some(image) => Some(image.to_owned()),
            None => None
        }
    }

    pub fn previous_image(&mut self) -> Option<String> {
        if self.current_image == 0 {
            return None;
        }

        if let Some(image) = self.images.get(self.current_image - 1) {
            self.current_image -= 1;
            return Some(image.to_owned());
        }
        
        None
    }

    pub fn next_image(&mut self) -> Option<String> {
        if self.current_image + 1 == self.images.len() {
            return None;
        }

        if let Some(image) = self.images.get(self.current_image + 1) {
            self.current_image += 1;
            return Some(image.to_owned());
        }
        
        None
    }
}

fn read_images_in_folder(folder: &str) -> Vec<String> {
    let mut images = vec![];

    for entry in read_dir(folder).expect("Failed to read folder") {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                continue;
            }

            if let Some(name) = entry.file_name().to_str() {
                let name = format!("{}/{}", folder, name);
                images.push(name);
            }
        }
    }
    
    images
}

fn select_images(images: Vec<String>, number_of_images_to_select: usize) -> Vec<String> {
    let images = shuffle_images(images);

    if number_of_images_to_select > images.len() {
        return images[..images.len()].to_vec();
    }

    images[..number_of_images_to_select].to_vec()
}

fn shuffle_images(images: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();

    let mut images = images;
    images.shuffle(&mut rng);

    images
}