use std::fs::read_dir;
use core::fmt::Debug;

use super::settings::Settings;

#[derive(Debug, Clone)]
pub struct Session {
    pub images: Vec<String>,
    pub current_image: usize,

    // duration_timer
    // pause_timer
}

impl Session {
    pub fn new() -> Self {
        Self {
            images: vec![],
            current_image: 0,
        }
    }

    pub fn from(settings: &Settings) -> Self {
        let images = read_images_in_folder(&settings.folder);
        let images = select_images(images, settings.images_number);
        
        Self {
            current_image: 0,
            images,
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
    images
}