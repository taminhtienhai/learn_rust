trait TupleAdd3<Item> {
    fn add(&self, item: Item) -> (Item, Item, Item);
}

impl <Item: Clone> TupleAdd3<Item> for (Item, Item) {

    fn add(&self, item: Item) -> (Item, Item, Item) {
        (self.0.clone(), self.1.clone(), item)
    }
}

fn main() {
    let t = (String::from("a"), String::from("b"));

    let t2 = t.add(String::from("c"));

    println!("{t:?}");
}