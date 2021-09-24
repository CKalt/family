use std::mem;

struct Person<'a> {
    name: &'a str,
    age: i32,
    children: Vec<Person<'a>>,
}
impl<'a> Person<'a> {
    fn new(name: &'a str, age: i32) -> Person<'a> {
        Person {
            name: name,
            age,
            children: Vec::new(),
        }
    }
    fn add(&mut self, name: &'a str, age: i32) -> &mut Person<'a> {
        self.children.push(
            Self::new(name, age)
        );
        self
    }
    fn show(&self) {
        self.show_w_tab(0);
    }
    fn show_w_tab(&self, tab: usize) {
        println!("{:>1$}, {age} yrs old",
            self.name, tab + self.name.len(), age=self.age);
    }
    fn show_r(&self, tab: usize) {
        self.show_w_tab(tab);
        for c in self.children.iter() {
            c.show_r(tab + 4);
        }
    }
    fn show_family_tree(&self) {
        self.show_r(0)
    }
    /*
    fn collect_r<'b>(&'a self, result: &'b mut Vec<&'a Person<'a>>,
            filter: fn(&'a Person) -> bool) {
        if filter(self) {
            result.push(self);
        }
        for c in self.children.iter() {
            c.collect_r(result, filter);
        }
    }
    fn collect(&'a self, filter: fn(&Person) -> bool) -> Vec<&'a Self> {
        let mut result: Vec<&Person> = Vec::new();
        self.collect_r(&mut result, filter);
        result
    }
    */
}

fn main() {
    let mut tree1 = Person::new("Ruth", 120);
    tree1
        .add("Pat", 91)
        .add("John", 89);
        
    tree1.children[0]
        .add("Jim", 65)
        .add("Chuck", 65);
        
    tree1.children[1]
        .add("Stan", 57)
        .add("Anne", 55);
        
    tree1.children[1].children[0]
        .add("Mary", 20);

    tree1.children[1].children[1]
        .add("Helena", 21)
        .add("Peter", 19);

    /*
    let fifties = tree1.collect(|p| p.age >= 50 && p.age < 60);
    println!("fifties...");
    for p in fifties.iter() {
        p.show();
    }
    */

    let mut tree2 = Person::new("Murial", 91);
    tree2
        .add("Maya", 55)
        .add("Matt", 59);
        
    tree2.children[0]
        .add("Julia", 26)
        .add("Andria", 28);

    tree2.children[0].children[0]
        .add("Tom", 2);

    println!("Before Swap");

    tree1.show_family_tree();
    tree2.show_family_tree();

    let swap_target1 = &mut tree1.children[1].children[0];
    let swap_target2 = &mut tree2.children[0].children[0];

    print!("swap target 1 is: "); swap_target1.show();
    print!("swap target 2 is: "); swap_target2.show();

    mem::swap(swap_target1, swap_target2);

    println!("After Swap");
    tree1.show_family_tree();
    tree2.show_family_tree();

}
