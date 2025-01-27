#[derive(Debug, Clone, Default)]
pub struct OrderBookState {
    pub bids: HashMap<String, f64>,
    pub asks: HashMap<String, f64>,
}


pub static ORDER_BOOK: InitCell<Arc<RwLock<OrderBookState>>> = InitCell::new();
// let order_book = Arc::new(RwLock::new(OrderBookState::default()));
        // ORDER_BOOK.set(order_book.clone());