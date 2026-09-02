use std::ptr;

#[derive(Debug, Clone, Copy)]
struct Contact {
    name: [u8; 20],
    phone: [u8; 20],
}
const MAX_CONTACTS: usize = 100;
fn add_contract(contacts: *mut Contact, count: *mut i32) {
    
}

pub fn example() {
    let mut count: i32 = 0;
    let mut contacts: [Contact; MAX_CONTACTS] = [Contact {
        name: [0; 20],
        phone: [0; 20],
    }; MAX_CONTACTS];
    add_contract(contacts.as_mut_ptr(), &mut count as *mut i32);
}
