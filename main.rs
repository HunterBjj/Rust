use std::io;

fn modify_surname(mut last_name: String) -> String {

 let last_len = last_name.len();
 let mut last_slice = (&last_name[last_len - 4..]).to_string();
 
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
  last_name.push('а');
}
	return last_name;
}


fn modify_name(mut first_name: String) -> String {

 let first_len = first_name.len();
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
 else if first_slice == "ий" {
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('я');
    first_name = first_slice;

}
else if first_slice == "ра" {
    first_slice = (&first_name[..first_len - 2]).to_string();
    first_slice.push('ы');
    first_name = first_slice;

}
else {
    first_name.push('а');
}
	return first_name;
}


fn modify_middle(mut middle_name: String) -> String {

 let middle_len = middle_name.len();
 let mut middle_slice = (&middle_name[middle_len - 4..]).to_string();
 if middle_slice == "на"
{
    middle_slice = (&middle_name[..middle_len - 2]).to_string();
    middle_slice.push('ы');
    middle_name = middle_slice;
}
else {
    middle_name.push('а');
    
}
	return middle_name;
}

// Тест на Имя (5 тестов) 

#[test]
fn test1_modify_name()
{
	let s = String::from("Женя");
	assert_eq!(modify_name(s), "Жени");

}

#[test]
fn test2_modify_name()
{
	let s = String::from("Илья");
	assert_eq!(modify_name(s), "Ильи");

}

#[test]
fn test3_modify_name()
{
	let s = String::from("Сергей");
	assert_eq!(modify_name(s), "Сергея");

}

#[test]
fn test4_modify_name()
{
	let s = String::from("Артём");
	assert_eq!(modify_name(s), "Артёма");

}

#[test]
fn test5_modify_name()
{
	let s = String::from("Анастасия");
	assert_eq!(modify_name(s), "Анастасии");

}

// Тест Фамилия (5 тестов)

#[test]
fn test1_modify_surname()
{
	let s = String::from("Полонский");
	assert_eq!(modify_surname(s), "Полонского");

}

#[test]
fn test2_modify_surname()
{
	let s = String::from("Огнёва");
	assert_eq!(modify_surname(s), "Огнёвой");

}

#[test]
fn test3_modify_surname()
{
	let s = String::from("Степаненко");
	assert_eq!(modify_surname(s), "Степаненко");

}

#[test]
fn test4_modify_surname()
{
	let s = String::from("Конев");
	assert_eq!(modify_surname(s), "Конева");

}

#[test]
fn test5_modify_surname()
{
	let s = String::from("Талашко");
	assert_eq!(modify_surname(s), "Талашко");

}

// Тест на Отчество (5 тестов)

#[test]
fn test1_modify_middle()
{
	let s = String::from("Петровна");
	assert_eq!(modify_middle(s), "Петровны");

}

#[test]
fn test2_modify_middle()
{
	let s = String::from("Александрович");
	assert_eq!(modify_middle(s), "Александровича");

}

#[test]
fn test3_modify_middle()
{
	let s = String::from("Сергеевич");
	assert_eq!(modify_middle(s), "Сергеевича");

}

#[test]
fn test4_modify_middle()
{
	let s = String::from("Петрович");
	assert_eq!(modify_middle(s), "Петровича");

}

#[test]
fn test5_modify_middle()
{
	let s = String::from("Никитич");
	assert_eq!(modify_middle(s), "Никитича");

}

fn main() {

	println!("Введите своё ФИО ");
 	let mut full_name  = String::new();
	io::stdin().read_line(&mut full_name).expect("Failed");
	let token:Vec<&str> = full_name.split_whitespace().collect();
	let mut first_name = token[1].to_string();
	let mut last_name = token[0].to_string();
	let mut middle_name = token[2].to_string();
	let result_firstname = modify_name(first_name);
	let result_middlename = modify_middle(middle_name);
	let result_lastname = modify_surname(last_name);
	
	println!("Результат: {} {} {}", result_lastname, result_firstname, result_middlename);
}
