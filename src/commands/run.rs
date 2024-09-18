use std::fs;

use ab_glyph::{FontArc, PxScale};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    ClientBuilder,
};

use crate::{
    types::{Item, Query},
    Config,
};

pub async fn run() -> anyhow::Result<()> {
    let mut config = match Config::get_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(err);
        }
    };

    let get_items_query = [
        r#"
        query GetBoardItemsByPerson {
          boards(ids: 4539781298) {
            items_page(
              query_params: {rules: [{column_id: "person", compare_value: [""#,
        &config.person_name, // Insert dynamic value here
        r#""], operator: contains_text}]}
            ) {
              items {
                name
                column_values {
                  text
                  type
                }
              }
            }
          }
        }
        "#,
    ]
    .concat();

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").expect("lol bro fuck you"),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&config.api_key).expect("lol bro fuck you mfker"),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Couldn't get client");

    let res = client
        .post("https://api.monday.com/v2")
        .json(&Query {
            query: get_items_query,
        })
        .send()
        .await
        .unwrap();

    let response = res
        .json::<serde_json::Value>()
        .await
        .expect("There was an issue getting the data");

    // Get all the current working on it items
    let items = Item::items_from_response(response)
        .expect("Couldn't get items")
        .into_iter()
        .filter(|item| {
            if let (Some(person), Some(status)) =
                (item.column_values.get(1), item.column_values.get(5))
            {
                person.text == Some("Suryansh Srivastava".to_string())
                    && status.text == Some("Working on it".to_string())
            } else {
                false
            }
        })
        .collect::<Vec<Item>>();

    let image_width = config.output_image.width;
    let image_height = config.output_image.height;

    // Load the image
    let mut img = ImageBuffer::from_pixel(image_width, image_height, Rgba([0, 0, 0, 255]));

    // Load the font
    let font = fs::read(config.font.path)
        .expect("Couldn't load the font. Please recheck the path given in your config.");
    let font = FontArc::try_from_vec(font).expect("Failed to load font");

    // Define the text properties
    let scale = PxScale::from(50.0); // Font size

    // let mut start_height = 170;
    // let increment = 50;
    let width_offset = ((config.width_offset_perc / 100.0) * image_width as f32) as i32;

    // Draw the text onto the image
    draw_text_mut(
        &mut img,
        Rgba([255u8, 255u8, 255u8, 255u8]), // Black color with full opacity
        width_offset,
        100,
        scale, // Pass the PxScale directly here
        &font, // Pass the font as a reference
        "Luganodes Todos:",
    );

    for item in items {
        let out_str = format!("{}", item);
        draw_text_mut(
            &mut img,
            Rgba([255u8, 255u8, 255u8, 255u8]), // Black color with full opacity
            width_offset,
            config.start_height,
            scale, // Pass the PxScale directly here
            &font, // Pass the font as a reference
            &out_str,
        );
        config.start_height += config.height_increment;
    }

    config.start_height += 50;

    draw_text_mut(
        &mut img,
        Rgba([255u8, 255u8, 255u8, 255u8]), // Black color with full opacity
        width_offset,
        config.start_height,
        scale, // Pass the PxScale directly here
        &font, // Pass the font as a reference
        "Personal:",
    );

    let todos_file = fs::read_to_string(config.todos_path).expect("Couldn't load personal file");
    let todos = todos_file.split("\n");

    config.start_height += 60;

    for todo in todos {
        draw_text_mut(
            &mut img,
            Rgba([255u8, 255u8, 255u8, 255u8]), // Black color with full opacity
            width_offset,
            config.start_height,
            scale, // Pass the PxScale directly here
            &font, // Pass the font as a reference
            todo,
        );
        config.start_height += config.height_increment;
    }

    // Save the output image
    img.save(config.output_image.path)
        .expect("Failed to save image");

    Ok(())
}
