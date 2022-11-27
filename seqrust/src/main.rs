use rusqlite::{params, Connection, Result};
use std::fs::File;
// use std::io::Read;
// use std::io::{self, BufReader};
// use std::path::Path;
// use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};
// extern crate url;
// extern crate seal;

// use url::{Url, ParseError};
// use seal::pair::{
//     Alignment, AlignmentSet, InMemoryAlignmentMatrix, Step, NeedlemanWunsch, // SmithWaterman,
// };

#[derive(Debug)]
struct SQ {
    id: i32,
    name: String,
    sequence: String,
}

fn load_db(filename: &str, conn: &mut Connection) -> Result<()> {
    let file = File::open(&filename).expect("bad file");

    let mut reader = BufReader::new(file);
    // let mut line = String::new();
    // let len = reader.read_line(&mut line).expect("something went wrong");

    // println!("{len}{line}");
    for line in reader.lines(){
        let mut s = line.unwrap();
        let mut v: Vec<&str>  = s.split("\t").collect();
        if v.len() == 2 {
            let mut name = v[0];
            let mut s = v[1];
            // println!("{}: {}", name, s);
            insert_record(name.to_string(),s.to_string(),conn);
        }

        // print_type_of(&s)
    }

   
    Ok(())
}

fn insert_record(name: String, sequence: String, conn: &mut Connection) -> Result<()>{

    let tx = conn.transaction()?;

    tx.execute(
        "CREATE TABLE seqs (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            sequence BLOB
        )",
        (), // empty list of parameters.
    )?;
    
    // for line in reader.lines(){
        let sq = SQ {
            id: 0, 
            name: name,
            sequence: sequence,           
            // name: "AF1203".to_string(),
            // sequence: "TGTACACAGGTA ATGCTGAACGGCAG CCGCCACCAGAACTTGG  CGTACAC".chars()
            //     .filter(|ch| ! ch.is_whitespace()).collect(),
        };
        tx.execute(
            "INSERT INTO seqs (name, sequence) VALUES (?1, ?2)",
            (&sq.name, &sq.sequence),
        )?;
    // }
    tx.commit();
    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> Result<()>{

    let filename = "../test_files/sseeds.txt";

    let mut conn =  Connection::open_in_memory()?;


    load_db(&filename, &mut conn).expect("Not work");
    
    let mut stmt = conn.prepare("SELECT id, name, sequence FROM seqs")?;
    let sq_iter = stmt.query_map([], |row| {
        Ok(SQ {
            id: row.get(0)?,
            name: row.get(1)?,
            sequence: row.get(2)?,
        })
    })?;

    for s in sq_iter {
        println!("Found sequence {:?}", s.unwrap());
    }
    Ok(())
}
// fn orig_code () {
//     let str_x = "The quick brown fox jumps over the lazy dog.";
//     let str_y = "The brown dog jumps over the very lazy snail.";

//     let strategy = NeedlemanWunsch::new(1, -1, -1, -1);
//     // Alternatively:
//     // let strategy = SmithWaterman::new(2, -1, -1, -1);

//     let sequence_x: Vec<char> = str_x.chars().collect();
//     let sequence_y: Vec<char> = str_y.chars().collect();
//     let set: AlignmentSet<InMemoryAlignmentMatrix> =
//         AlignmentSet::new(sequence_x.len(), sequence_y.len(), strategy, |x, y| {
//             sequence_x[x] == sequence_y[y]
//         })
//         .unwrap();

//     let print_alignment = |alignment: Alignment| {
//         for step in alignment.steps() {
//             match step {
//                 Step::Align { x, y } => {
//                     if sequence_x[x] == sequence_y[y] {
//                         print!("=")
//                     } else {
//                         print!("!")
//                     }
//                 }
//                 Step::Delete { .. } => print!("-"),
//                 Step::Insert { .. } => print!("+"),
//             }
//         }
//         println!("\n");
//     };

//     println!("Local alignment:");
//     let local_alignment = set.local_alignment();
//     print_alignment(local_alignment);

//     println!("Global alignment:");
//     let global_alignment = set.global_alignment();
//     print_alignment(global_alignment);

//     // Local alignment:
//     // ====------======!=!================+++++=====
//     //
//     // Global alignment:
//     // ====------======!=!================+++++=====!!!++=
// }