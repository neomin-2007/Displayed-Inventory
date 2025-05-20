use macroquad::{prelude::*, ui::widgets::Texture};
use std::{fs, path::Path};

pub struct TextureState {
    pub is_ready: bool,
    pub textures: Vec<(String, Texture2D)>,
}

pub struct Item {
    display_name: String,
    amount: i32,
}

pub struct Inventory {
    stored_items: Vec<Item>,
    max_storage: f32,
}

impl TextureState {
    pub async fn new() -> Self {
        let mut textures_vec = Vec::new();
        let mut is_ready = true;
        let textures_dir = "textures";

        // Verifica se o diretório existe
        if !Path::new(textures_dir).exists() {
            eprintln!("Directory '{}' not found!", textures_dir);
            return TextureState {
                is_ready: false,
                textures: Vec::new(),
            };
        }

        // Lê o conteúdo do diretório
        match fs::read_dir(textures_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        
                        // Verifica se é um arquivo e tem extensão válida
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                if ext == "png" || ext == "jpg" || ext == "jpeg" {
                                    let file_name = path.to_string_lossy().into_owned();
                                    
                                    match load_texture(&file_name).await {
                                        Ok(texture) => {
                                            let name = path.file_stem()
                                                .unwrap_or_default()
                                                .to_string_lossy()
                                                .into_owned();
                                            textures_vec.push((name, texture));
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to load texture {}: {}", file_name, e);
                                            is_ready = false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to read textures directory: {}", e);
                is_ready = false;
            }
        }

        TextureState {
            is_ready: is_ready && !textures_vec.is_empty(),
            textures: textures_vec,
        }
    }

    // Método auxiliar para buscar textura pelo nome
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.iter()
            .find(|(n, _)| n == name)
            .map(|(_, texture)| texture)
    }
}

impl Item {

    pub fn new(name: &str) -> Item {
        Item {
            display_name: name.to_owned(),
            amount: 1,
        }
    }

}

impl Inventory {

    pub fn new() -> Inventory {
        Inventory {
            stored_items: Vec::new(),
            max_storage: 36.0,
        }
    }

    pub fn render(&self, textures: &TextureState) {
            let square_scale = 64.0;
            let items_per_row = 9;
            let padding = 2.0;
            let start_x = 120.0;
            let start_y = 275.0;
        
            // Desenha todos os slots primeiro
            for slot in 0..self.max_storage as i32 {
                let row = slot / items_per_row;
                let col = slot % items_per_row;
        
                let x = start_x + (col as f32 * (square_scale + padding));
                let y = start_y + (row as f32 * (square_scale + padding));
        
                draw_rectangle(x, y, square_scale, square_scale, GRAY);
            }
    
            // Depois desenha os itens em seus slots
            for (slot, item) in self.stored_items.iter().enumerate() {
                let row = slot as i32 / items_per_row;
                let col = slot as i32 % items_per_row;
    
                let x = start_x + (col as f32 * (square_scale + padding));
                let y = start_y + (row as f32 * (square_scale + padding));
    
                if let Some(texture) = textures.get_texture(&item.display_name) {
                    draw_texture_ex(
                        texture,
                        x + 8.0,  // Centraliza a textura no slot
                        y + 8.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(square_scale - 16.0, square_scale - 16.0)),
                            ..Default::default()
                        },
                    );
                    draw_text(&item.amount.to_string(), x + 8.0, y + 64.0, 24.0, WHITE);
                }
    
                // Desenha a quantidade se maior que 1
                if item.amount > 1 {
                    draw_text(
                        &item.amount.to_string(),
                        x + square_scale - 16.0,
                        y + square_scale - 8.0,
                        16.0,
                        WHITE,
                    );
                }
            }
    
            // Verificação de clique
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
    
                for slot in 0..self.max_storage as i32 {
                    let row = slot / items_per_row;
                    let col = slot % items_per_row;
        
                    let x = start_x + (col as f32 * (square_scale + padding));
                    let y = start_y + (row as f32 * (square_scale + padding));
        
                    if mouse_x >= x && mouse_x <= x + square_scale &&
                       mouse_y >= y && mouse_y <= y + square_scale {
                        println!("Clicked slot {}!", slot);
                        if let Some(item) = self.stored_items.get(slot as usize) {
                            println!("Item: {} (x{})", item.display_name, item.amount);
                        }
                    }
                }
            }
        }

    pub fn contains_item(&self, item_name: &str) -> bool {
        return self.stored_items.iter()
        .any(|i| i.display_name.contains(&item_name));
    }

    pub fn get_by_name(&mut self, item_name: &str) -> Option<&mut Item> {
        return self.stored_items.iter_mut()
        .find(|i| i.display_name == item_name);
    }

    pub fn add_item(&mut self, item: Item) {
        if let Some(stored_item) = self.get_by_name(&item.display_name) {
            stored_item.amount += item.amount;
        } else {
            self.stored_items.push(item);
        }
    }
}