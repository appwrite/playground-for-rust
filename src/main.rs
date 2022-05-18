use std::{collections::HashMap, path::PathBuf};

use appwrite::{client::{Client, ParamType}};
use colored::Colorize;
use std::time::{UNIX_EPOCH};

fn main() {
    let mut client = Client::new();

    client.set_endpoint("https://8080-appwrite-appwrite-tvywm1tzw03.ws-eu43.gitpod.io/v1");
    client.set_key("62b844915ffda3218cded724ca3c84ca67353a3b18fc711dfb41a539af3b6d19cdf759b8efa3c262f05e0fb6951adf43dab7059526d69788a67d83455e8d1ea8e3b36f6ccec4a7bfb645388dda575b501b1a6edc23c73d41e8691800fa4e826c08ad6a37db427e79ec088a8fd20461e4619675b820663bb22034d88719826d3d");
    client.set_project("626a5dfa03fa579769d8");

    let collection_id = create_collection(&client);
    std::thread::sleep(std::time::Duration::from_secs(2)); // wait for attributes to be computed
    list_collection(&client);
    list_attributes(&client, &collection_id);


    // get_account(&client);
    let document_id = add_document(&client, &collection_id);
    list_documents(&client, &collection_id);
    delete_document(&client, &collection_id, &document_id);
    delete_collection(&client, &collection_id);

    // Storage API
    let bucket_id = create_bucket(&client);
    list_buckets(&client);

    let file_id = upload_file(&client, &bucket_id);
    list_files(&client, &bucket_id);

    delete_file(&client, &bucket_id, &file_id);
    delete_bucket(&client, &bucket_id);

    // Users API
    let user_id = create_user(&client);
    list_users(&client);
    delete_user(&client, &user_id);

    // Functions API
    let function_id = create_function(&client);
    list_functions(&client);
    upload_deployment(&client, &function_id);

    // Wait for deployment to build
    std::thread::sleep(std::time::Duration::from_secs(2));

    execute_sync(&client, &function_id);
    execute_async(&client, &function_id);
    delete_function(&client, &function_id);

    println!("{}", "Successfully Ran Playground!".bright_green());
}

fn create_collection(client: &Client) -> String {
    println!("{}", "Running Create Collection API...".bright_green());
    let database = appwrite::services::Database::new(client);

    let collection = match database.create_collection("unique()", "Movies", "document", &["role:all"], &["role:all"]) {
        Ok(collection) => collection,
        Err(err) => panic!("{:?}", err),
    };

    match database.create_string_attribute(&collection.id, "name", 100, true, Some(""), Some(false)) {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    };

    match database.create_integer_attribute(&collection.id, "release_year", true, Some(0), None, None, None) {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", collection);

    return collection.id;
}

fn list_collection(client: &Client) {
    println!("{}", "Running List Collections API...".bright_green());
    let database = appwrite::services::Database::new(client);

    let collections = match database.list_collections(None, None, None, None, None, None) {
        Ok(collections) => collections,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", collections);
}

fn list_attributes(client: &Client, collection_id: &str) {
    println!("{}", "Running List Attributes API...".bright_green());
    let database = appwrite::services::Database::new(client);

    let attributes = match database.list_attributes(collection_id) {
        Ok(attributes) => attributes,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", attributes);
}

fn get_account(client: &Client) {
    println!("{}", "Running Get Account API...".bright_green());
    let account = appwrite::services::Account::new(client);

    let account = match account.get() {
        Ok(account) => account,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", account);
}

fn add_document(client: &Client, collection_id: &str) -> String{
    println!("{}", "Running Add Document API...".bright_green());
    let database = appwrite::services::Database::new(client);

    let movie = match database.create_document(collection_id, "unique()", Some(HashMap::from([
        ("name".to_string(), ParamType::String("Spider-Man".to_string())),
        ("release_year".to_string(), ParamType::Number(1920))

    ])), Some(&["role:all"]), Some(&["role:all"])) {
        Ok(movie) => movie,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", movie);

    return movie.id;
}

fn list_documents(client: &Client, collection_id: &str) {
    println!("{}", "Running List Documents API...".bright_green());
    let database = appwrite::services::Database::new(client);

    let documents = match database.list_documents(collection_id, None, None, None, None, None, None, None) {
        Ok(documents) => documents,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", documents);
}

fn delete_document(client: &Client, collection_id: &str, document_id: &str) {
    println!("{}", "Running Delete Document API...".bright_green());
    let database = appwrite::services::Database::new(client);

    match database.delete_document(collection_id, document_id) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn delete_collection(client: &Client, collection_id: &str) {
    println!("{}", "Running Delete Collection API...".bright_green());
    let database = appwrite::services::Database::new(client);

    match database.delete_collection(collection_id) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn create_bucket(client: &Client) -> String {
    println!("{}", "Running Create Bucket API...".bright_green());
    let storage = appwrite::services::Storage::new(client);

    let bucket = match storage.create_bucket("unique()", 
        "All Files", 
        "bucket", 
        Some(&["role:all"]), 
        Some(&["role:all"]),
        None, Some(50000000), None, None, None) {
            Ok(bucket) => bucket,
            Err(err) => panic!("{:?}", err),
        };

    println!("{:?}", bucket);

    return bucket.id;
}

fn upload_file(client: &Client, bucket_id: &str) -> String {
    println!("{}", "Running Upload File API...".bright_green());
    let storage = appwrite::services::Storage::new(client);

    let file = PathBuf::from("./resources/nature.jpg");

    let file_upload = match storage.create_file(bucket_id, "unique()", file, Some(&["role:all"]), Some(&["role:all"])) {
        Ok(file) => file,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", file_upload);
    return file_upload.id;
}

fn list_buckets(client: &Client) {
    println!("{}", "Running List Buckets API...".bright_green());
    let storage = appwrite::services::Storage::new(client);

    let buckets = match storage.list_buckets(None, None, None, None, None, None) {
        Ok(buckets) => buckets,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", buckets);
}

fn list_files(client: &Client, bucket_id: &str) {
    println!("{}", "Running List Files API...".bright_green());
    let storage = appwrite::services::Storage::new(client);

    let files = match storage.list_files(bucket_id, None, None, None, None, None, None) {
        Ok(files) => files,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", files);
}

fn delete_file(client: &Client, bucket_id: &str, file_id: &str) {
    println!("{}", "Running Delete File API...".bright_green());
    let storage = appwrite::services::Storage::new(client);

    match storage.delete_file(bucket_id, file_id) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn delete_bucket(client: &Client, bucket_id: &str) {
    println!("{}", "Running Delete Bucket API...".bright_green());
    let storage = appwrite::services::Storage::new(client);

    match storage.delete_bucket(bucket_id) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn create_user(client: &Client) -> String {
    println!("{}", "Running Create User API...".bright_green());
    let users = appwrite::services::Users::new(client);

    let now = std::time::SystemTime::now();

    let user = match users.create("unique()", &format!("{:?}@test.com", now.duration_since(UNIX_EPOCH).expect("Time went backwards!")), "password", Some("Brian")) {
        Ok(user) => user,
        Err(err) => panic!("{:?}", err),
    };

    return user.id;
}

fn list_users(client: &Client) {
    println!("{}", "Running List Users API...".bright_green());
    let users = appwrite::services::Users::new(client);

    match users.list(None, None, None, None, None, None) {
        Ok(data) => println!("{:?}", data),
        Err(err) => panic!("{:?}", err),
    };
}

fn delete_user(client: &Client, user_id: &str) {
    println!("{}", "Running Delete User API...".bright_green());
    let users = appwrite::services::Users::new(client);

    match users.delete(user_id) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn create_function(client: &Client) -> String {
    println!("{}", "Running Create Function API...".bright_green());
    let function = appwrite::services::Functions::new(client);

    let result = match function.create("unique()", "Node Hello World", &["role:all"], "node-16.0", None, None, None, None) {
        Ok(result) => result,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", result);
    return result.id;
}

fn list_functions(client: &Client) {
    println!("{}", "Running List Functions API...".bright_green());
    let function = appwrite::services::Functions::new(client);

    match function.list(None, None, None, None, None, None) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn upload_deployment(client: &Client, function_id: &str) {
    println!("{}", "Running Upload Deployment API...".bright_green());
    let function = appwrite::services::Functions::new(client);

    match function.create_deployment(function_id, "index.js", PathBuf::from("./resources/code.tar.gz"), true) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn execute_sync(client: &Client, function_id: &str) {
    println!("{}", "Running Execute Function API (sync)...".bright_green());
    let function = appwrite::services::Functions::new(client);

    match function.create_execution(function_id, None, Some(false)) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn execute_async(client: &Client, function_id: &str) {
    println!("{}", "Running Execute Function API (async)...".bright_green());
    let function = appwrite::services::Functions::new(client);

    let result = match function.create_execution(function_id, None, Some(true)) {
        Ok(result) => result,
        Err(err) => panic!("{:?}", err),
    };

    let execution_id = result.id;

    // wait a moment for execution to finish.
    std::thread::sleep(std::time::Duration::from_secs(2));

    match function.get_execution(function_id, &execution_id) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}

fn delete_function(client: &Client, function_id: &str) {
    println!("{}", "Running Delete Function API...".bright_green());
    let function = appwrite::services::Functions::new(client);

    match function.delete(function_id) {
        Ok(result) => println!("{:?}", result),
        Err(err) => panic!("{:?}", err),
    };
}