use rds::linked_list::LinkedList;

fn main() {
    // let mut s: Stack<i32> = Stack::<i32>::new();
    // s.push(5);
    // s.push(16);
    //
    // println!("{:?}", s);
    // println!("{}", s.len());
    // println!("{:?}", s.pop());
    // println!("{:?}", s.pop());
    // println!("{:?}", s.pop());
    // println!("{}", s.len());
    //
    // let mut q = Queue::<i32>::new();
    //
    // q.enqueue(32);
    //
    // println!("{:?}", q);
    // println!("{:?}", q.dequeue());
    // println!("{:?}", q.dequeue());
    // println!("{}", q.len());

    // let mut q = LinkedQueue::<i32>::new();
    // q.enqueue(32);
    // q.enqueue(16);
    // q.enqueue(19);
    //
    // println!("{:?}", q);
    // println!("{:?}", q.dequeue());
    // println!("{:?}", q.dequeue());
    // println!("{:?}", q.dequeue());
    // println!("{:?}", q.dequeue());
    // println!("{:?}", q.dequeue());
    // println!("{:?}", q);

    let mut l = LinkedList::<i32>::new();
    // l.insert(65);
    l.insert(65);
    l.push(7);
    l.push(6);
    l.insert(95);


    println!("{:?}", l);
    for i in l.into_iter() {
        println!("{:?}", i);
    }
}
