//
// Show casing SRP

pub struct UserManager {
    name: String,
}

impl UserManager {
    fn new(name: String) -> Self {
        UserManager {
            name: String::from("name"),
        }
    }

    fn update_user(&mut self, new_name: String) {
        self.name = new_name;
        println!("update user");
    }
}

// show casing OCP
//
pub trait PaymentMethod {
    fn process_payment(self, amount: f64) -> bool;
}

pub struct CashPayment {
    amount: f64,
}

pub struct CreditCardPayment {
    amount: f64,
}

impl CashPayment {
    fn new(amount: f64) -> Self {
        CashPayment { amount }
    }
}

impl CreditCardPayment {
    fn new(amount: f64) -> Self {
        CreditCardPayment { amount }
    }
}

impl PaymentMethod for CashPayment {
    fn process_payment(self, amount: f64) -> bool {
        true
    }
}

impl PaymentMethod for CreditCardPayment {
    fn process_payment(self, amount: f64) -> bool {
        true
    }
}

// Show casing LSP
//
pub trait Bird {
    fn fly(self);
}

pub struct Hummingbird {
    name: String,
}

impl Bird for Hummingbird {
    fn fly(self) {}
}

pub struct Ostrich {
    name: String,
}

impl Bird for Ostrich {
    // violates the LSP
    fn walk(self) {}
}

pub trait FlyingBird {
    fn fly(self);
}

pub trait NonFlyingBird {
    fn walk(self);
}

impl NonFlyingBird for Ostrich {
    // violates the LSP
    fn walk(self) {}
}

// show casing ISP
//
pub trait Worker {
    fn work(self);
    fn eat(self);
} //

pub struct Coder {
    name: String,
}

pub struct AiCoder {
    name: String,
}

impl Worker for Coder {
    fn work(self) {}
    fn eat(self) {}
}

impl Worker for AiCoder {
    fn work(self) {}
    fn eat(self) {} // violates ISP
}

pub trait MessageSender {
    pub fn send_message(self, message: String);
}

pub struct SmsSender {
    name: String,
}

impl MessageSender for SmsSender {
    fn send_message(self, message: String) {}
}
pub struct NotificationService {
    name: String,
}

impl NotificationService {
    fn send_notification<T>(&self, message: String, sender: T)
    where
        T: MessageSender,
    {
    }
}
