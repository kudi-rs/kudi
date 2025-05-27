use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use kudi::DepInj;

#[derive(Debug, DepInj)]
#[target(Person)]
struct PersonState<'a, S: ?Sized>
where
    S: AsRef<str>,
{
    name: &'a S,
}

impl<'a, S: ?Sized> Clone for PersonState<'a, S>
where
    S: AsRef<str>,
{
    fn clone(&self) -> Self {
        Self { name: self.name }
    }
}

#[derive(Debug)]
struct Container<'a, S: ?Sized + AsRef<str>> {
    person: PersonState<'a, S>,
}

impl<'a, S> Clone for Container<'a, S>
where
    S: ?Sized + AsRef<str>,
{
    fn clone(&self) -> Self {
        Self {
            person: self.person.clone(),
        }
    }
}

impl<'a, S> AsRef<PersonState<'a, S>> for Container<'a, S>
where
    S: ?Sized + AsRef<str>,
{
    fn as_ref(&self) -> &PersonState<'a, S> {
        &self.person
    }
}

impl<'a, S> AsMut<PersonState<'a, S>> for Container<'a, S>
where
    S: ?Sized + AsRef<str>,
{
    fn as_mut(&mut self) -> &mut PersonState<'a, S> {
        &mut self.person
    }
}

impl<'a, S> From<Container<'a, S>> for PersonState<'a, S>
where
    S: ?Sized + AsRef<str>,
{
    fn from(value: Container<'a, S>) -> Self {
        value.person
    }
}

#[test]
fn test_generics() {
    let person = PersonState { name: "Alice" };
    let mut container = Container {
        person: person.clone(),
    };
    let person_inj: Person<'_, str, Container<'_, str>> = Person::inj(container.clone());
    // Test basic inj/prj methods
    let _: Container<'_, str> = person_inj.prj();

    // Test reference methods
    let person_inj_ref: &Person<'_, str, Container<'_, str>> = Person::inj_ref(&container);
    let _: &Container<'_, str> = person_inj_ref.prj_ref();

    // Test mutable reference methods
    let person_inj_ref_mut: &mut Person<'_, str, Container<'_, str>> =
        Person::inj_ref_mut(&mut container);
    let _: &mut Container<'_, str> = person_inj_ref_mut.prj_ref_mut();

    // Test Box methods
    let person_inj_box: Box<Person<'_, str, Container<'_, str>>> =
        Person::inj_box(Box::new(container.clone()));
    let _: Box<Container<'_, str>> = person_inj_box.prj_box();

    // Test Rc methods
    let person_inj_rc: Rc<Person<'_, str, Container<'_, str>>> =
        Person::inj_rc(Rc::new(container.clone()));
    let _: Rc<Container<'_, str>> = person_inj_rc.prj_rc();

    // Test Arc methods
    let person_inj_arc: Arc<Person<'_, str, Container<'_, str>>> =
        Person::inj_arc(Arc::new(container.clone()));
    let _: Arc<Container<'_, str>> = person_inj_arc.prj_arc();

    // Test Pin reference methods
    let person_inj_pin_ref: Pin<&Person<'_, str, Container<'_, str>>> =
        Person::inj_pin_ref(Pin::new(&container));
    let _: Pin<&Container<'_, str>> = person_inj_pin_ref.prj_pin_ref();

    // Test Pin mutable reference methods
    let person_inj_pin_ref_mut: Pin<&mut Person<'_, str, Container<'_, str>>> =
        Person::inj_pin_ref_mut(Pin::new(&mut container));
    let _: Pin<&mut Container<'_, str>> = person_inj_pin_ref_mut.prj_pin_ref_mut();

    // Test Pin Box methods
    let person_inj_pin_box: Pin<Box<Person<'_, str, Container<'_, str>>>> =
        Person::inj_pin_box(Box::pin(container.clone()));
    let _: Pin<Box<Container<'_, str>>> = person_inj_pin_box.prj_pin_box();

    // Test Pin Rc methods
    let person_inj_pin_rc: Pin<Rc<Person<'_, str, Container<'_, str>>>> =
        Person::inj_pin_rc(Rc::pin(container.clone()));
    let _: Pin<Rc<Container<'_, str>>> = person_inj_pin_rc.prj_pin_rc();

    // Test Pin Arc methods
    let person_inj_pin_arc: Pin<Arc<Person<'_, str, Container<'_, str>>>> =
        Person::inj_pin_arc(Arc::pin(container.clone()));
    let _: Pin<Arc<Container<'_, str>>> = person_inj_pin_arc.prj_pin_arc();

    // Test Deref trait
    assert_eq!(person.name, Person::inj_ref(&container).name);

    // Test DerefMut trait
    assert_eq!(person.name, Person::inj_ref_mut(&mut container).name);

    // Test From trait conversion
    let person_inj: Person<'_, str, Container<'_, str>> = Person::inj(container);
    let _: PersonState<'_, str> = PersonState::<'_, str>::from(person_inj);
}
