use rocket::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, FileField};

#[post("/submit", data = "<data>")]
pub fn submit(content_type: &ContentType, data: Data) -> &'static str {
    let mut options = MultipartFormDataOptions::new();
    //options.allowed_fields.push(MultipartFormDataField::file("empire").content_type_by_string(Some(mime::JSON)).unwrap());

    let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let empire = multipart_form_data.files.get("empire");

    if let Some(empire) = empire {
        match empire {
            FileField::Single(file) => {
                let _content_type = &file.content_type;
                let _file_name = &file.file_name;
                let _path = &file.path;

                println!("{:?}{:?}{:?}", _content_type, _file_name, _path);
                // You can now deal with the uploaded file. The file will be delete automatically when the MultipartFormData instance is dropped. 
                // If you want to handle that file by your own, instead of killing it, just remove it out from the MultipartFormData instance.
            }
            FileField::Multiple(_files) => {
                println!("fuck");
                // Because we only put one "photo" field to the allowed_fields, this arm will not be matched.
            }
        }
    }
    "ok"
}
