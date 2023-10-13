fn make_document(text: &str) -> Vec<String> {
    let mut return_vec = Vec::new();
    let split_doc = text.split("\n\n");
    for line in split_doc {
        return_vec.push(line.to_string());
    }
    return_vec
}

fn rank_documents(docs: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    // make new vec to modify
    let mut return_vec = Vec::new();
    for doc in docs {
        return_vec.push(doc.to_vec());
    }

    for n in 0..docs.len() {
        for docnum in 0..docs.len() - 1 {
            // if it has less paragraphs that the next doc, swap them
            if return_vec[docnum].len() < return_vec[docnum + 1].len(){
                return_vec.swap(docnum, docnum + 1);
            }
        }
    }
    return_vec
}

fn main() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
The bustle in a house\n\
The morning after death\n\
Is solemnest of industries\n\
Enacted upon earth,—\n\
\n\
The sweeping up the heart,\n\
And putting love away\n\
We shall not want to use again\n\
Until eternity.\n\
";
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs

    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);
}

#[test]
fn test() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
The bustle in a house\n\
The morning after death\n\
Is solemnest of industries\n\
Enacted upon earth,—\n\
\n\
The sweeping up the heart,\n\
And putting love away\n\
We shall not want to use again\n\
Until eternity.\n\
";
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}
