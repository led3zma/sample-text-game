use csv::{ReaderBuilder, StringRecord};
use std::{collections::HashMap, fs, vec};

const FILE_NAME: &str = "story.csv";
const INITIAL_TAG: &str = "INICIO";

#[derive(Debug)]
struct StoryData {
    event: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<StoryData>,
}

impl StoryData {
    pub fn build(row: StringRecord) -> StoryData {
        let event = row.get(0).unwrap().trim().to_string();
        let tag = row.get(1).unwrap().trim().to_string();
        let text = row.get(2).unwrap().trim().to_string();
        let life: i32 = row.get(3).unwrap().trim().parse().unwrap_or(0);
        StoryData {
            event,
            tag,
            text,
            life,
            options: vec![],
        }
    }
}

fn main() {
    let mut life = 100;
    let mut actual_tag = &String::from(INITIAL_TAG);
    let mut last_story_record = String::new();

    let contents = fs::read_to_string(FILE_NAME).unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(contents.as_bytes());

    let mut story_data: HashMap<String, StoryData> = HashMap::new();
    rdr.records().for_each(|row| {
        let story_record = StoryData::build(row.unwrap());
        if story_record.event == "SITUACION" {
            let tag = story_record.tag.clone();
            story_data.insert(tag.clone(), story_record);
            last_story_record = tag;
        } else if story_record.event == "OPCION" {
            if let Some(record) = story_data.get_mut(&last_story_record) {
                (*record).options.push(story_record);
            }
        }
    });

    // Game Loop
    loop {
        println!("Tienes {life} de vida");
        if let Some(record) = story_data.get(actual_tag) {
            println!("{:?}", record.text);
            record
                .options
                .iter()
                .enumerate()
                .for_each(|(index, option)| println!("[{index}] {}", option.text));

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(selected_option) = record.options.get(selection) {
                actual_tag = &selected_option.tag;
            } else {
                println!("Opción invalida")
            }

            life += record.life;
        } else {
            break;
        }
        if life <= 0 {
            println!("Has perdido!");
            break;
        }
    }
}
