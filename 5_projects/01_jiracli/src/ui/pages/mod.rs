use std::any::Any;
use std::rc::Rc;

use itertools::Itertools;
use anyhow::Result;
use anyhow::anyhow;

use crate::db::JiraDatabase;
use crate::models::Action;

mod page_helpers;
// use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");
        
        //Read in the database from disk.
        let db_state = self.db.read_db()?;

        //Set the col width
        let width_col_id = 12;
        let width_col_name = 33;
        let width_col_status = 17;

        //Loop through the sorted epic list
        for epic_key in db_state.epics.keys().sorted(){
            let curr_epic = db_state.epics.get(epic_key).ok_or_else(|| anyhow!("could not find epic!"))?;

            let eid = page_helpers::get_column_string(&epic_key.to_string(), width_col_id);
            let ename = page_helpers::get_column_string(&curr_epic.name, width_col_name);
            let estate = page_helpers::get_column_string(format!("{}", curr_epic.status).as_str(), width_col_status);

            println!("{:width_col_id$}| {:width_col_name$}| {:width_col_status$}", eid, ename, estate);
        }

        println!();
        println!();
        println!("[q] quit | [c] create epic | [:id:] navigate to epic");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {        
        // Match against the user input and return the corresponding action. If the user input was invalid return None.
        match input{
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            input if input.parse::<u32>().is_ok() => {
                //Read in the database from disk.
                let db_state = self.db.read_db()?;
                
                let epic_id = input.parse::<u32>().unwrap();
                if db_state.epics.contains_key(&epic_id){
                    Ok(Some(Action::NavigateToEpicDetail{epic_id}))
                }
                else {
                    Ok(None)
                }                
            },
            _ => Ok(None),
        }
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        
        //Set the col width
        let width_col_eid = 6;
        let width_col_ename = 13;
        let width_col_edesc = 28;
        let width_col_estatus = 13;
        let width_col_sid = 12;
        let width_col_sname = 33;
        let width_col_sstatus = 17;

        let db_state = self.db.read_db()?;
        let epic = db_state.epics.get(&self.epic_id).ok_or_else(|| anyhow!("Could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        // Print out epic details using get_column_string()
        let eid = page_helpers::get_column_string(&self.epic_id.to_string(), width_col_eid);
        let ename = page_helpers::get_column_string(&epic.name, width_col_ename);
        let edesc = page_helpers::get_column_string(&epic.description, width_col_edesc);
        let estatus = page_helpers::get_column_string(format!("{}", epic.status).as_str(), width_col_estatus);
  
        println!("{:width_col_eid$}| {:width_col_ename$}| {:width_col_edesc$}| {:width_col_estatus$}", eid, ename, edesc, estatus);

        println!();
        println!();
        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");

        let stories = &db_state.stories;
        
        //Loop through all the sorted stories and print out
        for story_id in &epic.stories{
            let curr_story = db_state.stories.get(story_id).ok_or_else(||println!("Invalid Story ID in Epic: {}", story_id)).unwrap();

            let sid = page_helpers::get_column_string(&story_id.to_string(), width_col_sid);
            let sname = page_helpers::get_column_string(&curr_story.name, width_col_sname);
            let sstatus = page_helpers::get_column_string(format!("{}", curr_story.status).as_str(), width_col_sstatus);

            println!("{:width_col_sid$}| {:width_col_sname$}| {:width_col_sstatus$}", sid, sname, sstatus);
        }

        println!();
        println!();
        println!("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story\n\n");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        
        // Match against the user input and return the corresponding action. If the user input was invalid return None.        
        match input{
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus { epic_id: self.epic_id})),
            "d" => Ok(Some(Action::DeleteEpic { epic_id: self.epic_id})),
            "c" => Ok(Some(Action::CreateStory { epic_id: self.epic_id})),
            input if input.parse::<u32>().is_ok() => {
                // Read in the database from disk.
                let db_state = self.db.read_db()?;
                let epic = db_state.epics.get(&self.epic_id).ok_or_else(|| anyhow!("Could not find epic!"))?;

                // Parse the Story ID.
                let story_id = input.parse::<u32>().unwrap();
                
                // Check if the Story ID exist in the db and epic
                if db_state.stories.contains_key(&story_id) && epic.stories.contains(&story_id){
                    Ok(Some(Action::NavigateToStoryDetail {epic_id: self.epic_id, story_id: input.parse::<u32>().unwrap()}))
                } else {
                    Ok(None)
                }
            },
            _ => Ok(None),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        //Set the col width
        let width_col_id = 6;
        let width_col_name = 13;
        let width_col_desc = 28;
        let width_col_status = 13;

        let db_state = self.db.read_db()?;
        let story = db_state.stories.get(&self.story_id).ok_or_else(|| anyhow!("could not find story!"))?;

        println!("------------------------------ STORY ------------------------------");
        println!("  id  |     name     |         description         |    status    ");
        
        // Print out story details using get_column_string()
        let sid = page_helpers::get_column_string(&self.story_id.to_string(), width_col_id);
        let sname = page_helpers::get_column_string(&story.name, width_col_name);
        let sdesc = page_helpers::get_column_string(&story.description, width_col_desc);
        let sstatus = page_helpers::get_column_string(format!("{}", story.status).as_str(), width_col_status);
  
        println!("{:width_col_id$}| {:width_col_name$}| {:width_col_desc$}| {:width_col_status$}", sid, sname, sdesc, sstatus);
        
        println!();
        println!();
        println!("[p] previous | [u] update story | [d] delete story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // Match against the user input and return the corresponding action. If the user input was invalid return None.
        match input{
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus { story_id: (self.story_id) })),
            "d" => Ok(Some(Action::DeleteStory { epic_id: (self.epic_id), story_id: (self.story_id) })),
            _ => Ok(None),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::test_utils::MockDB};
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert_eq!(page.draw_page().is_ok(), true);
        }
        
        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(page.handle_input(&valid_epic_id).unwrap(), Some(Action::NavigateToEpicDetail { epic_id: 1 }));
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = EpicDetail { epic_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateEpicStatus { epic_id: 1 }));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteEpic { epic_id: 1 }));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateStory { epic_id: 1 }));
            assert_eq!(page.handle_input(&story_id.to_string()).unwrap(), Some(Action::NavigateToStoryDetail { epic_id: 1, story_id: 2 }));
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        } 
    }

    mod story_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let _ = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateStoryStatus { story_id }));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteStory { epic_id, story_id }));
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        } 
    }
}