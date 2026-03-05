use crate::todo::{Todo, TodoStore};
use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

#[rpc(server, namespace = "todo")]
pub trait TodoApi {
    #[method(name = "list")]
    async fn list(&self) -> Result<Vec<Todo>, ErrorObjectOwned>;

    #[method(name = "add")]
    async fn add(&self, text: String) -> Result<Todo, ErrorObjectOwned>;

    #[method(name = "toggle")]
    async fn toggle(&self, id: u64) -> Result<Todo, ErrorObjectOwned>;

    #[method(name = "remove")]
    async fn remove(&self, id: u64) -> Result<bool, ErrorObjectOwned>;

    #[method(name = "clearCompleted")]
    async fn clear_completed(&self) -> Result<u64, ErrorObjectOwned>;
}

impl TodoApiServer for TodoStore {
    async fn list(&self) -> Result<Vec<Todo>, ErrorObjectOwned> {
        Ok(TodoStore::list(self))
    }

    async fn add(&self, text: String) -> Result<Todo, ErrorObjectOwned> {
        Ok(TodoStore::add(self, text))
    }

    async fn toggle(&self, id: u64) -> Result<Todo, ErrorObjectOwned> {
        TodoStore::toggle(self, id)
            .ok_or_else(|| ErrorObjectOwned::owned(-32602, "Todo not found", None::<()>))
    }

    async fn remove(&self, id: u64) -> Result<bool, ErrorObjectOwned> {
        Ok(TodoStore::remove(self, id))
    }

    async fn clear_completed(&self) -> Result<u64, ErrorObjectOwned> {
        Ok(TodoStore::clear_completed(self))
    }
}
