use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub total_items: u64,
    pub current_page: u64,
    pub total_pages: u64,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total_items: u64,
    pub current_page: u64,
    pub total_pages: u64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            meta: None,
        }
    }
}

impl<T> ApiResponse<Vec<T>> {
    pub fn paginated(paginated_data: PaginatedData<T>) -> Self {
        Self {
            success: true,
            data: paginated_data.data,
            meta: Some(PaginationMeta {
                total_items: paginated_data.total_items,
                current_page: paginated_data.current_page,
                total_pages: paginated_data.total_pages,
            }),
        }
    }
}
