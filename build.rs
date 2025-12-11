extern crate embed_resource;

fn main() {
    // 版本信息
    embed_resource::compile("./resource/resource.rc", embed_resource::NONE);

    // 兼容 Windows 7、Windows XP
    #[cfg(not(debug_assertions))]
    thunk::thunk();
}
