
/*
큐 

선입후출 */
struct Queue<T> {
    items:Vec<T>,
}

impl<T>Queue <T> {
    fn new()->Self {
        Queue {items:Vec::new()}
    }

    fn enqueue(&mut self,item:T){
        self.items.push(item);
    }

    fn dequeue(&mut self)->Option<T>{
        if self.is_empty(){
            None
        }else {
            Some(self.items.remove(0))

        }
    }

    fn is_empty(&self)->bool {
        self.items.is_empty()
    }
}






/* medium 
new
remove
is_empty
*/


