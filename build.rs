// generated by `sqlx migrate build-script`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        //.out_dir("src/google")  // you can change the generated code's location
        .compile(
            &["proto/quay/rfq.proto"],
            &["proto/quay"], // specify the root location to search proto dependencies
        )
        .unwrap();

    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/quay/session.proto"],
            &["proto/quay"], // specify the root location to search proto dependencies
        )
        .unwrap();
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");
    Ok(())
}
