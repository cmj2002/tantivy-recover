// This file read all documents into a csv file from the index
extern crate tantivy;
use tantivy::Index;
use tqdm::tqdm;

fn main() {
    println!("Start processing index...");
    
    // Read existing index
    let index = Index::open_in_dir("index").unwrap();
    let index_reader = index.reader().unwrap();
    let searcher = index_reader.searcher();
    let schema = index.schema();

    let csv_file = std::fs::File::create("output.csv").unwrap();
    let mut csv_writer = csv::Writer::from_writer(csv_file);

    // Write header
    let mut header = Vec::new();
    for (_field,field_entry) in schema.fields() {
        header.push(field_entry.name().to_string());
    }
    csv_writer.write_record(header).unwrap();


    let segment_headers = searcher.segment_readers();
    println!("{} segments found", segment_headers.len());


    for segment_reader in segment_headers {
        let id = segment_reader.segment_id().uuid_string();
        
        let store_reader = segment_reader.get_store_reader(1048576)
            .expect("Failed to get store reader");

        // Print number of documents in the segment
        println!("Processing Segment {}: {} documents", id, segment_reader.num_docs());

        for doc in tqdm(store_reader.iter(segment_reader.alive_bitset())){
            let doc = doc.unwrap();
            let mut doc_vec = Vec::new();
            for (field,field_entry) in schema.fields() {
                match field_entry.field_type(){
                    tantivy::schema::FieldType::Str(_) => {
                        let text = doc.get_first(field).unwrap().as_text().unwrap();
                        doc_vec.push(text.to_string());
                    },
                    tantivy::schema::FieldType::U64(_) => {
                        let num = doc.get_first(field).unwrap().as_u64().unwrap();
                        doc_vec.push(num.to_string());
                    },
                    // raise error if other type
                    _ => panic!("Unsupported field type. Only Str and U64 are supported."),
                }
            }
            csv_writer.write_record(doc_vec).unwrap();
        }
    }
    println!("Done, documents written to output.csv")
}
