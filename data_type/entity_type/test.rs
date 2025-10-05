mod service2 {
    pub enum ServiceLayer1 {
        AirConditioner,
        WashingMachine,
        AnyMachine,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ServiceLayer1 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ServiceLayer1::AirConditioner => "AirConditioner",
                    ServiceLayer1::WashingMachine => "WashingMachine",
                    ServiceLayer1::AnyMachine => "AnyMachine",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ServiceLayer1 {
        #[inline]
        fn clone(&self) -> ServiceLayer1 {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ServiceLayer1 {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ServiceLayer1 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ServiceLayer1 {
        #[inline]
        fn eq(&self, other: &ServiceLayer1) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ServiceLayer1 {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ServiceLayer1 {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    pub enum ServiceLayer2 {
        AirConditionerFixing,
        AirConditionerCleaning,
        WashingMachineFixing,
        WashingMachineCleaning,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ServiceLayer2 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ServiceLayer2::AirConditionerFixing => "AirConditionerFixing",
                    ServiceLayer2::AirConditionerCleaning => "AirConditionerCleaning",
                    ServiceLayer2::WashingMachineFixing => "WashingMachineFixing",
                    ServiceLayer2::WashingMachineCleaning => "WashingMachineCleaning",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ServiceLayer2 {
        #[inline]
        fn clone(&self) -> ServiceLayer2 {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ServiceLayer2 {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ServiceLayer2 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ServiceLayer2 {
        #[inline]
        fn eq(&self, other: &ServiceLayer2) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ServiceLayer2 {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ServiceLayer2 {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl ServiceLayer2 {
        /// Returns the parent [`ServiceLayer1`] for the current [`ServiceLayer2`].
        pub fn layer1(&self) -> ServiceLayer1 {
            match self {
                ServiceLayer2::AirConditionerFixing => ServiceLayer1::AirConditioner,
                ServiceLayer2::AirConditionerCleaning => ServiceLayer1::AirConditioner,
                ServiceLayer2::WashingMachineFixing => ServiceLayer1::WashingMachine,
                ServiceLayer2::WashingMachineCleaning => ServiceLayer1::WashingMachine,
            }
        }
    }
    impl ServiceLayer1 {
        /// Returns a slice containing all child [`ServiceLayer2`] variants.
        pub fn layer2(&self) -> &[ServiceLayer2] {
            match self {
                ServiceLayer1::AirConditioner => {
                    &[
                        ServiceLayer2::AirConditionerFixing,
                        ServiceLayer2::AirConditionerCleaning,
                    ]
                }
                ServiceLayer1::WashingMachine => {
                    &[
                        ServiceLayer2::WashingMachineFixing,
                        ServiceLayer2::WashingMachineCleaning,
                    ]
                }
                ServiceLayer1::AnyMachine => &[],
            }
        }
    }
}
