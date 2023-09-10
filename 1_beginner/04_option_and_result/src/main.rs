fn main() {

    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));
    let v2 = vec![1,2,3];

    let s = &v[0]; //can panic
    // let s = v.remove(0);
    let t = v.get(0);

    if let Some(e) = t{
        println!("{e}")
    }

    for u in &mut v{
        u.push_str("!");
    }

    for u in &v{
        println!("{u}");
    }

    let mut v3 = vec![];
    for u in v{
        v3.push(u);
    }    

    let username = get_username(0);

    if let Some(name) = username {
        println!("{name}");
    }   
    else {
        println!("No user found.")
    }     
}

fn get_username(userid: u32) -> Option<String> {

    let query = 
        format!("GET username FROM users WHERE id ={userid}");

    //get username from database
    let db_result = query_db(query);
        
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty!"))
    } else {
        Ok(String::from("Ferris"))
    }
}