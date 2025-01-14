#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use structx::*;

    #[test]
    fn anonymous_struct() {
        let a = structx! { width :  800, height: 600 };
        let b = structx! { height:  600, width : 800 };
        let c = structx! { width : 1024, height: 768 };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn returns_anonymous_struct() {
        fn returns_structx(x: i32, y: i32) -> Structx! { x: i32, y: i32 } {
            structx! { x, y }
        }

        assert_eq!(returns_structx(3, 4), structx! { x:3, y:4 });

        #[derive(Debug, PartialEq)]
        struct Bar<T>(T);

        fn returns_generic_structx<T>(bar: Bar<T>) -> Structx! { bar: Bar<T>, baz: bool } {
            structx! { bar, baz: true }
        }

        assert_eq!(
            returns_generic_structx(Bar("bar")),
            structx! { bar: Bar("bar"), baz: true }
        );
    }

    #[test]
    fn nested_anonymous_struct() {
        let pixel = structx! {
            color: structx!{ red: 255u8, green: 0u8, blue: 0u8 },
            position: structx!{ x: 3u16, y: 4u16 },
        };

        fn returns_nested_structx(
            red: u8,
            green: u8,
            blue: u8,
            x: u16,
            y: u16,
        ) -> Structx! {
               color: Structx!{ red: u8, green: u8, blue: u8 },
               position: Structx!{ x: u16, y: u16 },
           } {
            structx! {
                color: structx!{ red, green, blue },
                position: structx!{ x, y },
            }
        }

        assert_eq!(returns_nested_structx(255, 0, 0, 3, 4), pixel);

        let recursive_structx = structx! { a: structx!{ a: "recursive structx" }, };

        fn returns_recursive_structx() -> Structx! { a: Structx!{ a: &'static str }} {
            structx! { a: structx!{ a: "recursive structx" }}
        }

        assert_eq!(returns_recursive_structx(), recursive_structx);
    }

    #[test]
    fn type_only_anonymous_struct() {
        #[allow(dead_code)]
        type Props<T, T1> = Structx! {
            items: Vec<Structx!{ name: T }>,
            products: Vec<Structx! { el: Structx! { field: T1 } }>
        };
    }

    #[test]
    fn recursively_nested_anonymous_struct() {
        let model = structx! {
            products: vec![
                structx! {
                    id: 0,
                    name: "Pullover"
                },
                structx! {
                    id: 1,
                    name: "T-Shirt"
                },
            ],
            session: structx! {
                user: structx! {
                    username: "xxtheusernamexx"
                },
                tokens: HashMap::from([("a0bd6d46-3324-4566-836b-96b3767b6295", structx! {
                    valid_until: 1684249334,
                    created: 1664249334
                })])
            }
        };

        fn returns_nested_structx(
            username: &str,
        ) -> Structx! {
               products: Vec<Structx!{ id: usize, name: &'static str }>,
               session: Structx! {
                   user: Structx! { username: &str },
                   tokens: HashMap<&str, Structx!{ valid_until: usize, created: usize }> }
           } {
            structx! {
                products: vec![
                    structx! {
                        id: 0,
                        name: "Pullover"
                    },
                    structx! {
                        id: 1,
                        name: "T-Shirt"
                    },
                ],
                session: structx! {
                    user: structx! {
                        username
                    },
                    tokens: HashMap::from([("a0bd6d46-3324-4566-836b-96b3767b6295", structx! {
                        valid_until: 1684249334,
                        created: 1664249334
                    })])
                }
            }
        }
        assert_eq!(returns_nested_structx("xxtheusernamexx"), model);
    }

    #[test]
    fn named_argsuments() {
        use structx::named_args::*;

        #[named_args]
        fn with_owned_args(x: i32, y: String) -> String {
            format!("{} {}", x, y)
        }

        #[named_args]
        fn with_borrowed_args<'a>(x: bool, y: &'a str) -> String {
            format!("{} {}", x, y)
        }

        assert_eq!(
            with_owned_args(args! { x: 3, y: "4".to_owned() }),
            "3 4".to_owned()
        );
        assert_eq!(
            with_borrowed_args(args! { x: true, y: "false" }),
            "true false".to_owned()
        );
    }

    #[test]
    fn test_pattern_matching() {
        let alpha = 42u8;
        let beta = true;
        let my_record = structx! {
            alpha,
            beta ,
            gamma: "Dancing Ferris",
        };
        match my_record {
            structx! { alpha, beta, gamma } => println!("{}, {}, {}", alpha, beta, gamma),
        }

        let structx! { alpha, beta, gamma } = my_record;
        println!("{}, {}, {}", alpha, beta, gamma);
    }

    #[test]
    fn test_struct_update_syntax() {
        let yellow = structx! { red: 0, green: 255, blue: 255 };
        let white = structx! { red: 255, ..yellow };
        assert_eq!(white, structx! { red: 255, green: 255, blue: 255 });
    }

    #[cfg(feature = "lens")]
    #[test]
    fn lens_test_nested() {
        use lens_rs::*;

        #[derive(Copy, Clone, Debug, Review, Prism)]
        enum Either<L, R> {
            #[optic]
            Left(L),
            #[optic]
            Right(R),
        }
        use Either::*;

        #[derive(Copy, Clone, Debug, Lens)]
        struct Tuple<A, B>(#[optic] A, #[optic] B);

        let mut x: (
            i32,
            Either<Tuple<Vec<Option<Structx! { a:String, b:i32 }>>, i32>, i32>,
        ) = (
            1,
            Left(Tuple(
                vec![
                    Some(structx! {
                        a : "a".to_string(),
                        b : 2,
                    }),
                    None,
                    Some(structx! {
                        a : 'c'.to_string(),
                        b : 3,
                    }),
                ],
                4,
            )),
        );

        x.preview_mut(optics!(_1.Left._1)).map(|x| *x *= 2);
        assert_eq!(x.preview_ref(optics!(_1.Left._1)), Some(&8));

        x.preview_mut(optics!(_1.Right)).map(|x: &mut i32| *x *= 2);
        assert_eq!(x.preview_ref(optics!(_1.Right)), None);

        *x.view_mut(optics!(_0)) += 1;
        assert_eq!(x.0, 2);

        x.traverse_mut(optics!(_1.Left._0._mapped.Some.a))
            .into_iter()
            .for_each(|s| *s = s.to_uppercase());
        assert_eq!(
            x.traverse(optics!(_1.Left._0._mapped.Some.a)),
            vec!["A".to_string(), "C".to_string()]
        );
    }
}
