use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use kudi::DepInj;

#[derive(Debug, DepInj, Clone)]
#[target(Person)]
struct PersonState {
    name: String,
}

#[derive(Debug, Clone)]
struct Container {
    person: PersonState,
}

impl AsRef<PersonState> for Container {
    fn as_ref(&self) -> &PersonState {
        &self.person
    }
}

impl AsMut<PersonState> for Container {
    fn as_mut(&mut self) -> &mut PersonState {
        &mut self.person
    }
}

impl From<Container> for PersonState {
    fn from(value: Container) -> Self {
        value.person
    }
}

#[test]
fn test_plain() {
    let person = PersonState {
        name: "Alice".into(),
    };
    let mut container = Container {
        person: person.clone(),
    };
    let person_inj: Person<Container> = Person::inj(container.clone());
    // Test basic inj/prj methods
    let _: Container = person_inj.prj();

    // Test reference methods
    let person_inj_ref: &Person<Container> = Person::inj_ref(&container);
    let _: &Container = person_inj_ref.prj_ref();

    // Test mutable reference methods
    let person_inj_ref_mut: &mut Person<Container> = Person::inj_ref_mut(&mut container);
    let _: &mut Container = person_inj_ref_mut.prj_ref_mut();

    // Test Box methods
    let person_inj_box: Box<Person<Container>> = Person::inj_box(Box::new(container.clone()));
    let _: Box<Container> = person_inj_box.prj_box();

    // Test Rc methods
    let person_inj_rc: Rc<Person<Container>> = Person::inj_rc(Rc::new(container.clone()));
    let _: Rc<Container> = person_inj_rc.prj_rc();

    // Test Arc methods
    let person_inj_arc: Arc<Person<Container>> = Person::inj_arc(Arc::new(container.clone()));
    let _: Arc<Container> = person_inj_arc.prj_arc();

    // Test Pin reference methods
    let person_inj_pin_ref: Pin<&Person<Container>> = Person::inj_pin_ref(Pin::new(&container));
    let _: Pin<&Container> = person_inj_pin_ref.prj_pin_ref();

    // Test Pin mutable reference methods
    let person_inj_pin_ref_mut: Pin<&mut Person<Container>> =
        Person::inj_pin_ref_mut(Pin::new(&mut container));
    let _: Pin<&mut Container> = person_inj_pin_ref_mut.prj_pin_ref_mut();

    // Test Pin Box methods
    let person_inj_pin_box: Pin<Box<Person<Container>>> =
        Person::inj_pin_box(Box::pin(container.clone()));
    let _: Pin<Box<Container>> = person_inj_pin_box.prj_pin_box();

    // Test Pin Rc methods
    let person_inj_pin_rc: Pin<Rc<Person<Container>>> =
        Person::inj_pin_rc(Rc::pin(container.clone()));
    let _: Pin<Rc<Container>> = person_inj_pin_rc.prj_pin_rc();

    // Test Pin Arc methods
    let person_inj_pin_arc: Pin<Arc<Person<Container>>> =
        Person::inj_pin_arc(Arc::pin(container.clone()));
    let _: Pin<Arc<Container>> = person_inj_pin_arc.prj_pin_arc();

    // Test Deref trait
    assert_eq!(person.name, Person::inj_ref(&container).name);

    // Test DerefMut trait
    assert_eq!(person.name, Person::inj_ref_mut(&mut container).name);

    // Test From trait conversion
    let person_inj: Person<Container> = Person::inj(container);
    let _: PersonState = PersonState::from(person_inj);
}
