use kudi::target;

#[target]
struct Person;

#[derive(Debug, Clone)]
struct Container;

#[test]
fn test_stateless() {
    let container = Container;

    // Test reference methods
    let person_inj_ref: &Person<Container> = Person::inj_ref(&container);
    let _: &Container = person_inj_ref.prj_ref();
}
