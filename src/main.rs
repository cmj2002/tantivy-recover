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

    // For version 1.1.0 and 1.2.0
    const CSV_FIELDS: [&str; 13] = [
        "id",
        "title",
        "author",
        "publisher",
        "extension",
        "filesize",
        "language",
        "year",
        "pages",
        "isbn",
        "ipfs_cid",
        "cover_url",
        "md5"
    ];

    let csv_file = std::fs::File::create("output.csv").unwrap();
    let mut csv_writer = csv::Writer::from_writer(csv_file);
    csv_writer.write_record(CSV_FIELDS).unwrap();


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
            for field_name in CSV_FIELDS {
                let field = schema.get_field(field_name).unwrap();
                let field_entry = schema.get_field_entry(field);
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
    // clean ups
    csv_writer.flush().unwrap();
    println!("Done, documents written to output.csv")
}
