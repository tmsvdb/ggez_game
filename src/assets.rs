
use ggez::graphics;

pub struct Assets {
    image_list: Vec<graphics::Image>,
    text_list: Vec<graphics::Text>,
}

impl Assets {
    pub fn new () -> Assets {
        Assets{
            image_list: Vec::new(),
            text_list: Vec::new(),
        }
    }

    pub fn add_image (&mut self, image: graphics::Image) -> usize {
        self.image_list.push(image);
        return self.image_list.len() - 1;
    }

    pub fn add_text (&mut self, text: graphics::Text) -> usize {
        self.text_list.push(text);
        return self.text_list.len() - 1;
    }

    pub fn get_image_at(&self, index: usize) -> Result<&graphics::Image, &str> {
        if index < self.image_list.len(){
            Ok(&self.image_list[index])
        }
        else {
            Err("Assets::get_image_at @param index: index out of range!")
        }
    }
    pub fn get_text_at(&self, index: usize) -> Result<&graphics::Image, &str> {
        if index < self.image_list.len(){
            Ok(&self.image_list[index])
        }
        else {
            Err("Assets::get_text_at @param index: index out of range!")
        }
    }
    /*
    pub fn get_asset_by_tag (&self, tag: String) -> Option<&Asset> {
        
        if let Some(index) = self.list.iter().position(|e| e.tag == tag)
        {
            Some(&self.list[index])
        }
        else {
            None
        }
    }
    */
}