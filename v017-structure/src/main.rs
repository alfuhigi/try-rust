struct Employee {
    name:String,
    company:String,
    age:u32
}


impl Employee {
    fn get_instance(name:String,company:String,age:u32) -> Employee {
        Employee { name: name, company: company, age: age }
    }
    fn display(&self) {
        println!("Name is :{} company is {} age is {}",self.name,self.company,self.age);
    }
}
fn main() {
    let emp1 = Employee{
        company:String::from("aramco"),
        name:String::from("alfuhigi"),
        age:99
    };
    let emp2 = Employee::get_instance( String::from("aramco"),
    String::from("alfuhigi"),
    99);

emp1.display();
    
let elder = who_is_elder(emp1,emp2);
println!("elder is:");
  display(elder);
   
}
fn display( emp:Employee) {
   println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}

fn who_is_elder (emp1:Employee,emp2:Employee)->Employee {
   if emp1.age>emp2.age {
      return emp1;
   } else {
      return emp2;
   }
}
