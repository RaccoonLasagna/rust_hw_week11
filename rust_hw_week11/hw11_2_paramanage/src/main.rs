use std::fs::File;
use std::io::{self, Read, Write};

fn make_document(text: &str) -> Vec<String> {
    let mut return_vec = Vec::new();
    let split_doc = text.split("\r\n\r\n");
    for line in split_doc {
        return_vec.push(line.to_string());
    }
    return_vec
}

fn rank_documents(docs: &Vec<Vec<String>>, doc_name: Vec<&str>) -> (Vec<Vec<String>>, Vec<String>){
    // make new vec to modify
    let mut docs_vec = Vec::new();
    for doc in docs {
        docs_vec.push(doc.to_vec());
    }
    let mut doc_name_vec = Vec::new();
    for docname in doc_name {
        doc_name_vec.push(docname.to_string());
    }

    for n in 0..docs.len() {
        for docnum in 0..docs.len() - 1 - n {
            // if it has less paragraphs that the next doc, swap them
            if docs_vec[docnum].len() < docs_vec[docnum + 1].len(){
                docs_vec.swap(docnum, docnum + 1);
                doc_name_vec.swap(docnum, docnum + 1);
            }
        }
    }
    (docs_vec, doc_name_vec)
}

fn read(file_path_vec: Vec<&str>) -> io::Result<()> {
    let mut doc_vec = Vec::new();
    for file_path in &file_path_vec{
        println!("file path: {}", file_path);
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let doc = make_document(&contents);
        doc_vec.push(doc);
    }
    let ordered_doc = rank_documents(&doc_vec, file_path_vec.clone());
    let mut html_string = "<style>
    table, td {
    border: 1px solid #000000;
    border-collapse: collapse;
}
</style>
<table>
  <tr>
    <th>File Name</th>
    <th>Paragraphs</th>
  </tr>".to_string();

    for docnum in 0..ordered_doc.0.len(){
        let formatted_string = format!("\n\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>", ordered_doc.1[docnum], ordered_doc.0[docnum].len());
        html_string += &formatted_string;
    }

    html_string += "\n</table>";
    let mut file = File::create("avgarea.html")?;
    file.write_all(html_string.as_bytes())?;

    println!("{:?}", ordered_doc);
    Ok(())
}

fn main(){
    let file_path_vec = vec!["textfiles\\bustle.txt", "textfiles\\fox.txt", "textfiles\\para3.txt"];
    println!("{:?}", read(file_path_vec));
}