pub trait CrudMutator{

    async fn create<T>(&self, model: T) {}
    async fn update<T>(&self, model: T) {}
    async fn delete<T>(&self, model: T) {}
}

//impl<T: sqlx_crud::traits::Crud>> CrudMutator for T{}


