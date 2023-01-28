mod web;

fn main() {
    async_global_executor::block_on(web::listen());
}
