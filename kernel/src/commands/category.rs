use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CategoryCommand {
    Create {
        name: String,
        ordering: i32,
    },
    UpdateName {
        name: String,
    },
    UpdateOrdering {
        ordering: i32,
    },
    Delete,
    AddProduct {
        ordered: i32,
        product_id: Uuid,
    },
    UpdateProductOrdering {
        ordered: i32,
        product_id: Uuid,
    },
}