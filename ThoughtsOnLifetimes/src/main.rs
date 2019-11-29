struct InputData<'a> {
    field: &'a u32,
}

struct OutputData<'b>(&'b u32);

fn foo1<'a, 'b>(input: &'a InputData<'b>) -> OutputData<'a> {
    // SS: We pass in input via reference, which has lifetime 'a.
    // Inside input, we have a reference that has (in general) a different lifetime 'b.
    // We make that explicit by specifying both here.

    // Since we bind OutputData's field to InputData's filed, we specify its lifetime
    // accordingly...
    OutputData(input.field)
}

fn foo2<'a, 'b>(input: &'a InputData<'b>) -> OutputData<'b> {
    // SS: We pass in input via reference, which has lifetime 'a.
    // Inside input, we have a reference that has (in general) a different lifetime 'b.
    // We make that explicit by specifying both here.

    // Since we bind OutputData's field to InputData's filed, we specify its lifetime
    // accordingly...
    OutputData(input.field)
}

fn test1() {
    let f = 76;
    let input = InputData { field: &f };
    let _foo_result = foo1(&input);
}

fn test2() {
    let f = 76;

    // SS: This fails because we bind the lifetime of OutputData's field
    // to the lifetime of InputData, NOT to the lifetime of InputData's
    // field, which is different because it is defined in the outer scope
    //    let _fo_result = {
    //        let input = InputData { field: &f };
    //        foo1(&input)
    //    };
}

fn test3() {
    let f = 76;

    // SS: This fails because we bind the lifetime of OutputData's field
    // to the lifetime of InputData, NOT to the lifetime of InputData's
    // field, which is different because it is defined in the outer scope
    let _foo_result: OutputData = {
        let input = InputData { field: &f };
        foo2(&input)
    };
}

fn foo3<'a, 'b>(input: &'a InputData<'b>) -> impl Fn() -> &'b u32 {
    // SS: Here, func captures the reference input.field, so the reference
    // must life at least as long as func does, or said differently, func
    // must not outlive input.field

    /* We cannot do

            let func = || input.field;

            as this gives error message

    error[E0623]: lifetime mismatch
      --> src/main.rs:73:46
       |
    73 | fn foo4<'a, 'b>(input: &'a InputData<'b>) -> impl Fn() -> &'b u32 {
       |                        -----------------     ^^^^^^^^^^^^^^^^^^^^
       |                        |                     |
       |                        |                     ...but data from `input` is returned here
       |                        this parameter and the return type are declared with different lifetimes...



            Instead, we have to bind input.field to a local variable and then MOVE that
            local variable into the closure, so the compiler doesn't complain that tmp
            does not life long enough...
            This is because tmp does out of scope at the end of this function, so that is we
            we have to move it.
        */
    let tmp: &'b u32 = input.field;
    let func = move || tmp;
    func
}

fn test4() {
    let f = 76;

    let _foo_result = {
        let input = InputData { field: &f };
        foo3(&input)
    };
}

struct InputDataMut<'a> {
    field: &'a mut u32,
}

impl<'a> InputDataMut<'a> {

    fn foo4<'b>(&'b mut self) -> impl FnMut() -> &'a u32 {
        let tmp: &'a mut u32 = self.field;
        let func = || {
//            *tmp = 10;
            tmp as &'a u32
        };
        func
    }

}


fn test5() {}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}
