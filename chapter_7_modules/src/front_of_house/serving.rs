fn take_order() {}

fn serve() {}

fn take_payment() {}

fn pay() {
    //relative path, but starting at ../
    super::hosting::get_check();
}
