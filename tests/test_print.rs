use megam_rustyprint::Printer;

#[test]
fn print() {

    let mut a = Printer::new();
    let mut b = Vec::new();
    b.push("Name".to_string());
    b.push("Email".to_string());
 		b.push("Twitter".to_string());
 	//	b.push("header5".to_string());
    a.set_header(b);

    let mut parent = Vec::new();

    let mut child1 = Vec::new();
    child1.push("Kishore".to_string());
    child1.push("nkishore@megam.io".to_string());
		child1.push("@indykish".to_string());

    let mut child2 = Vec::new();
    child2.push("Thomas".to_string());
    child2.push("thomasalrin@megam.io".to_string());
		child2.push("@thomasalrin".to_string());

    let mut child3 = Vec::new();
    child3.push("Rajthilak".to_string());
    child3.push("rajthilak@megam.io".to_string());
		child3.push("@rajthilak".to_string());

		let mut child4 = Vec::new();
    child4.push("Yeshwanth".to_string());
    child4.push("getyesh@megam.io".to_string());
		child4.push("@morpheyesh".to_string());

    parent.push(child1);
		parent.push(child2);
		parent.push(child3);
		parent.push(child4);

    a.set_body(parent);

		println!("{:?}", a);
   
}
