fn main (){
    let testobj = Test{str:"hello".to_string()};
    testobj.print();


}

struct Test {
    str : String
}
impl Test {
    fn new() -> Self{
        Test{ str : "".to_string()}
    }
}
impl SuperClass for Test {
    fn print(&self) {
        println!("{}", &self.str);
    }

}

trait SuperClass {
    fn print(&self ){
        println!("test from UpperClass");
        
    }
}
//trait のなかの関数の処理は呼ばれない
