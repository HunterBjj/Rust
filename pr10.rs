use std::io;

fn update_fio(mut full_name: String) -> String 
{
 let token:Vec<&str> = full_name.split_whitespace().collect();
 let mut first_name = token[1].to_string();
 let mut last_name = token[0].to_string();
 let mut middle_name = token[2].to_string();
 
 let middle_len = middle_name.len();
 let first_len = first_name.len();
 let last_len = last_name.len();

 let mut middle_slice = (&middle_name[middle_len - 4..]).to_string();
 let mut last_slice = (&last_name[last_len - 4..]).to_string();
 let mut first_slice = (&first_name[first_len - 4..]).to_string();

if first_slice == "ей" {
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('я');
    first_name = first_slice;
 }
 else if first_slice == "ия" {
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('и');
    first_name = first_slice;

 }
 else if first_slice == "га" {
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('и'); 
    first_name = first_slice;
 } 

 else if first_slice == "ша"{
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('и');
    first_name = first_slice;
 } 

 else if first_slice == "на"{
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('ы');
    first_name = first_slice;

 } 

 else if first_slice == "ья"{
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('и');
    first_name = first_slice;

 } 
 else if first_slice == "ня" {
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('и');
    first_name = first_slice;
 }
 else if first_slice == "да" {
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('ы');
    first_name = first_slice;
        
}
else {
    first_name.push('a');
}

if last_slice == "ко"{
         
}
else if last_slice == "ий" {
    last_slice = (&last_name[..last_len - 4]).to_string();
    last_slice.push_str("ого");
    last_name = last_slice;

}
else if last_slice == "ва" {
    last_slice = (&last_name[..last_len - 2]).to_string();
    last_slice.push_str("ой");
    last_name = last_slice;

}
else
{
  last_name.push('a');
}

if middle_slice == "на"
{
    middle_slice = (&middle_name[..middle_len - 2]).to_string();
    middle_slice.push('ы');
    middle_name = middle_slice;
}
else {
    middle_name.push('a');
}

full_name = last_name + " " + &first_name + " " +  &middle_name;
//println!("{}", full_name);
return full_name; 
 
}


fn main() {

	println!("Введите своё ФИО ");
 	let mut full_name  = String::new();
	io::stdin().read_line(&mut full_name).expect("Failed");
	let result = update_fio(full_name);
	println!("Результат: {}", result);
//	let fullname =  last_name + " " + &first_name + " " + &middle_name ;
//	println!(" Вы ввели: {:?}. Результат:",  result);

}
