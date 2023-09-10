#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::{Result, Context, anyhow};
use crate::models::{DBState, Epic, Story, Status};
pub struct JiraDatabase {
    pub database: Box<dyn Database>
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        JiraDatabase{
            database: Box::new(
                JSONFileDatabase{
                    file_path
                }
            )
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }
    
    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        //Read in the database from disk
        let mut db_state = self.database.read_db()?;

        //Get the current ID
        let curr_id = db_state.last_item_id + 1;

        //Add the new epic to the db
        db_state.epics.insert(curr_id, epic);

        //Modify and write the db state to disk.
        db_state.last_item_id = curr_id;
        self.database.write_db(&db_state)?;

        //Return result.
        Ok(curr_id)
    }
    
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        //Read in the database from disk
        let mut db_state = self.database.read_db()?;

        //Check if the epic id is valid.
        if !db_state.epics.contains_key(&epic_id){
            return Err(anyhow!("Invalid Epic Id."));
        }
        
        //Get the current ID
        let curr_id = db_state.last_item_id + 1;
        
        //Add the new story to the db
        db_state.stories.insert(curr_id, story);

        //Add the new story id to the correct epic.
        db_state.epics.entry(epic_id).and_modify(|epic| epic.stories.push(curr_id));

        //Modify and write the db state to disk.
        db_state.last_item_id = curr_id;
        self.database.write_db(&db_state)?;

        //Return result.
        Ok(curr_id)
    }
    
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        //Read in the database from disk
        let mut db_state = self.database.read_db()?;

        //Get the targeted Epic
        let tgt_epic = db_state.epics.get(&epic_id).ok_or_else(|| anyhow!("Invalid Epic Id."))?;

        //Check the story ids are valid. Sanity check
        let mut valid = true;        
        for story_id in &tgt_epic.stories{
            if !db_state.stories.contains_key(&story_id){
                valid = false;
            }
        }
        if !valid {
            return Err(anyhow!("Invalid Story Id in epic... Check Database before deleting..."));
        }

        //Delete the stories of the epic
        for story_id in &tgt_epic.stories{
            db_state.stories.remove(&story_id);
        }
        
        //Remove the epic and write the db state to disk.
        db_state.epics.remove(&epic_id);
        self.database.write_db(&db_state)?;

        //Return result.
        Ok(())
    }
    
    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        //Read in the database from disk
        let mut db_state = self.database.read_db()?;

        //Check if the story id is valid.
        if !db_state.stories.contains_key(&story_id){
            return Err(anyhow!("Invalid Story Id."));
        }

        //Check if the epic id is valid.
        if !db_state.epics.contains_key(&epic_id){
            return Err(anyhow!("Invalid Epic Id."));
        }
        
        //Remove the story from the db
        db_state.stories.remove(&story_id);

        //Remove the story id from the given epic. Assuming the epic contains the story...
        db_state.epics.entry(epic_id).and_modify(|epic| {
            epic.stories.retain(|curr_story_id| *curr_story_id!= story_id);
        });

        //Modify and write the db state to disk.
        self.database.write_db(&db_state)?;

        //Return result.
        Ok(())
    }
    
    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
         //Read in the database from disk
         let mut db_state = self.database.read_db()?;

         //Check if the epic id is valid.
         if !db_state.epics.contains_key(&epic_id){
            return Err(anyhow!("Invalid Epic Id."));
        }

        //Modify the status of the epic and write the db state to disk.
        db_state.epics.entry(epic_id).and_modify(|epic| epic.status = status);
        self.database.write_db(&db_state)?;

        //Return result.
        Ok(())
    }
    
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
         //Read in the database from disk
         let mut db_state = self.database.read_db()?;

         //Check if the story id is valid.
         if !db_state.stories.contains_key(&story_id){
            return Err(anyhow!("Invalid Story Id."));
        }

        //Modify the status of the story and write the db state to disk.
        db_state.stories.entry(story_id).and_modify(|story| story.status = status);
        self.database.write_db(&db_state)?;

        //Return result.
        Ok(())
    }
}

pub trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        
        //load json file into string
        let db_json = std::fs::read_to_string(&self.file_path)?;
        serde_json::from_str::<DBState>(&db_json).with_context(|| format!("Unable to deserialize json file into db: \n{db_json}\n"))
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let db_json = serde_json::to_string_pretty(db_state)?;
        std::fs::write(&self.file_path, &db_json).with_context(|| format!("Unable to write json db file into disk: \nPath: {}\n\n{db_json}\n", self.file_path))
    }
}

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;
    
    pub struct MockDB {
        last_written_state: RefCell<DBState>
    }

    impl MockDB {
        pub fn new() -> Self {
            Self { last_written_state: RefCell::new(DBState { last_item_id: 0, epics: HashMap::new(), stories: HashMap::new() }) }
        }    
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::MockDB;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic.clone());
        
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id), true);
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);
        
        let story_id = result.unwrap();

        let non_existent_epic_id = 999;
        
        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;
        
        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), false);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::Closed);
    }

    mod database {
        use std::collections::HashMap;
        use std::fs::{remove_file};
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/read_db_should_fail_with_invalid_json.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();

            let db = JSONFileDatabase { file_path: file_path.clone() };

            let result = db.read_db();

            remove_file(file_path).unwrap();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/read_db_should_parse_json_file.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();

            let db = JSONFileDatabase { file_path: file_path.clone() };

            let result = db.read_db();

            remove_file(file_path).unwrap();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/write_db_should_work.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();

            let db = JSONFileDatabase { file_path: file_path.clone() };

            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            remove_file(file_path).unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}