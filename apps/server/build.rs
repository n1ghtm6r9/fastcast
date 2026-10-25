fn main() {
    embed_resource::compile("assets/fastcast.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
