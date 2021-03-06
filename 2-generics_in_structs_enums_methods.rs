/* GENERIC TYPES: 
- to remove duplication of code, by using generics in structs, enums, and method definitions.    */ 

#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {

    
    
// 1) In STRUCT DEFINITIONS (in one or more fields using the <> syntax):

    struct Point <T> {      //to hold x and y coordinate values of any type - the Point<T> struct is generic over some type T - 
        x: T,       //both fields are that same type - 
        y: T, 
    }

    let integer= Point {x: 5, y: 10}; 
    let float= Point {x: 1.2, y: 5.3}; 

    // Struct with fields of different generic type: 
    struct Point2 <T, U> {
        x: T, 
        y: U, 
    }

    let integer_float= Point2 {x: 10, y: 10.0}; 

    
    
// 2) In ENUM DEFINITIONS - enums to hold generic data types in their variants:

    enum Option <T> {       //Option is a std lib enum generic over type T - 
        Some (T),       //variant Some that hold one value of type T, but could hold more - abstraction of having an optional value/s - 
        None,       //null variant - 
    }

    enum Result <T, E> {        //Std lib Result enum is generic over multiple types - 2 in this case - Good for error handling to return a value of type T or E - 
        Ok (T),         //Ok variant holds a value type T (e.g. T: std::fs::File when opening a file successfully) - 
        Err (E),        //Err holds a value type E (e.g. E: std::io::Error) - 
    }

    
    
// 3) In METHOD DEFINITIONS:

    impl <T> Point <T> {        //method y implemented on the struct Point with fields of type generic -
        fn y (&self) -> &T {        //method that returns a reference to the data in the field y of type T - 
            &self.y              
        }
    }

    let p= Point {x: 10, y: 20}; 
    println! ("Method y gives back the value y of p => p.y()= {}, and the value of the field y is also p.y= {}", p.y(), p.y); 

    // Implementing methods on Point <f32> instances only: 
    impl Point <f32> {
        fn distance_from_origin (&self) -> f32 {        //returns a concrete type f32 for the generic type parameter T - 
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    println! ("The distance between the origin and the point (5.3, 9.5) is: {}", Point{x:5.3, y:9.5}.distance_from_origin()); 

    // Implementing methods with parameters T and U:
    impl <T, U> Point2 <T, U> {
        fn mixup <V, W> (self, extra_point: Point2 <V, W>) -> Point2 <T, W> {       //mixup method with arguments self and extra_point - it takes ownership of both arguments - 
            Point2 {x: self.x, y: extra_point.y}        //generic parameters T and U are declared with impl and parameters V and W are declared with the method definition and only relevant there -
        }   
    }

    let p1= Point2 {x: 5, y: 10.6}; 
    let p2= Point2 {x: "hola!", y: 'j'}; 

    let p3= p1.mixup (p2);
    println! ("p3.x= {}, p3.y= {}", p3.x, p3.y); 
}
