pub trait PrimaryEducation {
  fn is_primary_pass(&self)->bool;
}
pub trait Bilingual {
  fn is_bilingual(&self)->bool;
}
pub trait YateemaKhana{
  fn is_orphan(&self)->bool;
}
#[derive(Debug)]
pub struct Child{
  name : String,
}
impl YateemaKhana for Child{
  fn is_orphan(&self)->bool{
      true
  }
}
impl PrimaryEducation for Child{
  fn is_primary_pass(&self)->bool{
      true
  }  
}
impl Bilingual for Child{
  fn is_bilingual(&self)->bool{
      true
  }  
}

pub fn adopt<T: PrimaryEducation + Bilingual>(child_1:T,child_2:T){
   if child_1.is_primary_pass() && child_2.is_bilingual() {
    println!("Home sweet home");     
  }
}

fn main() {
  let child_1 = Child{ name : String::from("Raju"), };
  let child_2 = Child{ name : String::from("Sabir"), };
  println!("{:#?}",child_1);
  println!("{:#?}",child_2);

  adopt(child_1,child_2);


    

}