#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Student{
    name: String,
    gpa: f32,
}

fn main(){
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5", 
        "Kyle 3.9",
        "Anatoliy 4.0",
    ];

    let good_students: Vec<Student> = 
        students.iter()
        .filter_map(|curr_student| {
            let mut stud_split = curr_student.split(' ');
            let name = stud_split.next()?.to_owned();
            let gpa_str = stud_split.next()?;

            let gpa_score = gpa_str.parse::<f32>().ok()?;

            if gpa_score >= 3.5{
                Some(Student{name, gpa: gpa_score})
            }else{
                None
            }                
        })        
        .collect(); //Above combinators are lazy. Nothing was done until this statement.

    // let good_students: Vec<Student> = 
    //     students.iter()
    //     .map(|curr_student| {
    //         let mut stud_split = curr_student.split(' ');
    //         let name = stud_split.next()?.to_owned();
    //         let gpa_str = stud_split.next()?;

    //         let gpa_score = gpa_str.parse::<f32>().ok()?;

    //         Some(Student{name, gpa: gpa_score})
    //     })
    //     .flatten()
    //     .filter(|curr_student| curr_student.gpa >= 3.5)
    //     .collect(); //Above combinators are lazy. Nothing was done until this statement.

    // ----------------------------------------------

    // let mut good_students: Vec<Student> = Vec::new();
    // for curr_student in students{
    //     let mut stud_split = curr_student.split(' ');
    //     let name = stud_split.next();
    //     let gpa_str = stud_split.next();

    //     if let (Some(name), Some(gpa_str)) = (name, gpa_str){
    //         let gpa_score = gpa_str.parse::<f32>().unwrap();
    //         let name = name.to_owned();

    //         if gpa_score >= 3.5 {
    //             good_students.push(Student{name, gpa: gpa_score});
    //         }
    //     }
    // }

    // ----------------------------------------------

    for gs in good_students{
        println!("{:?}", gs);
    }
}