pub trait CrudSelector{
    async fn list<T>(&self, model: T) {}
    async fn retrieve<T>(&self, model: T) {}
}
