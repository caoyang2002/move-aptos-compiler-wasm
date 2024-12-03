module addr::test {
  use aptos_std::debug::print;

  fun  test(){
    let a = 10;
    print(&a);
  }
}