use::std::rc::Rc;
use::std::cell::RefCell;

fn main() {
    // Smart pointers 
    // BOX<T> : Allow value on heap in save mode 
    // RC<T> : Enable multiple pointers to share ownership
    // RefCell<T> : Allow interior mutability when immutable reference is used

        let chest = Box::new(10);

        let shared_chest = Rc::new(RefCell::new(chest));

        **shared_chest.borrow_mut() += 10;
        **shared_chest.borrow_mut() += 5;

        println!("Gold : {}", shared_chest.borrow());



}
