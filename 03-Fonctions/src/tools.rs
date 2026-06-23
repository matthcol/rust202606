pub fn swap(a: &mut u64, b: &mut u64){
    let tmp = *a;
    *a = *b;
    *b = tmp
}