use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/",
        input: &["../data_model.proto"],
        includes: &[".."],
        customize: Customize {
            ..Default::default()
        },
    })
    .expect("protoc");
}
