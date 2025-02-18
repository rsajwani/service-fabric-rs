#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricApplicationHealthResult_Impl: Sized {
    fn get_ApplicationHealth(&self) -> *mut super::super::FABRIC_APPLICATION_HEALTH;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricApplicationHealthResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricApplicationHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationHealthResult_Vtbl {
        unsafe extern "system" fn get_ApplicationHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationHealth: get_ApplicationHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationManagementClient_Impl: Sized {
    fn BeginProvisionApplicationType(
        &self,
        applicationbuildpath: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionApplicationType(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginCreateApplication(
        &self,
        description: *const super::super::FABRIC_APPLICATION_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateApplication(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginUpgradeApplication(
        &self,
        upgradedescription: *const super::super::FABRIC_APPLICATION_UPGRADE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpgradeApplication(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetApplicationUpgradeProgress(
        &self,
        applicationname: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationUpgradeProgress(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricApplicationUpgradeProgressResult2>;
    fn BeginMoveNextApplicationUpgradeDomain(
        &self,
        progress: ::core::option::Option<&IFabricApplicationUpgradeProgressResult2>,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextApplicationUpgradeDomain(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginDeleteApplication(
        &self,
        applicationname: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteApplication(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginUnprovisionApplicationType(
        &self,
        applicationtypename: &::windows_core::PCWSTR,
        applicationtypeversion: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnprovisionApplicationType(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient {}
impl IFabricApplicationManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient_Vtbl {
        unsafe extern "system" fn BeginProvisionApplicationType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationbuildpath: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionApplicationType(
                ::core::mem::transmute(&applicationbuildpath),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndProvisionApplicationType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionApplicationType(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginCreateApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_APPLICATION_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateApplication(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateApplication(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginUpgradeApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            upgradedescription: *const super::super::FABRIC_APPLICATION_UPGRADE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpgradeApplication(
                ::core::mem::transmute_copy(&upgradedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpgradeApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpgradeApplication(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetApplicationUpgradeProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationUpgradeProgress(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationUpgradeProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationUpgradeProgress(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveNextApplicationUpgradeDomain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            progress: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextApplicationUpgradeDomain(
                ::windows_core::from_raw_borrowed(&progress),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMoveNextApplicationUpgradeDomain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextApplicationUpgradeDomain(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteApplication(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteApplication(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginUnprovisionApplicationType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows_core::PCWSTR,
            applicationtypeversion: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnprovisionApplicationType(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnprovisionApplicationType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnprovisionApplicationType(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginProvisionApplicationType: BeginProvisionApplicationType::<Identity, Impl, OFFSET>,
            EndProvisionApplicationType: EndProvisionApplicationType::<Identity, Impl, OFFSET>,
            BeginCreateApplication: BeginCreateApplication::<Identity, Impl, OFFSET>,
            EndCreateApplication: EndCreateApplication::<Identity, Impl, OFFSET>,
            BeginUpgradeApplication: BeginUpgradeApplication::<Identity, Impl, OFFSET>,
            EndUpgradeApplication: EndUpgradeApplication::<Identity, Impl, OFFSET>,
            BeginGetApplicationUpgradeProgress: BeginGetApplicationUpgradeProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetApplicationUpgradeProgress: EndGetApplicationUpgradeProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginMoveNextApplicationUpgradeDomain: BeginMoveNextApplicationUpgradeDomain::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextApplicationUpgradeDomain: EndMoveNextApplicationUpgradeDomain::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginDeleteApplication: BeginDeleteApplication::<Identity, Impl, OFFSET>,
            EndDeleteApplication: EndDeleteApplication::<Identity, Impl, OFFSET>,
            BeginUnprovisionApplicationType: BeginUnprovisionApplicationType::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUnprovisionApplicationType: EndUnprovisionApplicationType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricApplicationManagementClient10_Impl:
    Sized + IFabricApplicationManagementClient9_Impl
{
    fn BeginProvisionApplicationType3(
        &self,
        description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionApplicationType3(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient10 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricApplicationManagementClient10_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient10_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient10_Vtbl {
        unsafe extern "system" fn BeginProvisionApplicationType3<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionApplicationType3(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndProvisionApplicationType3<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionApplicationType3(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient9_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginProvisionApplicationType3: BeginProvisionApplicationType3::<Identity, Impl, OFFSET>,
            EndProvisionApplicationType3: EndProvisionApplicationType3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient10 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient8 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient9 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationManagementClient2_Impl:
    Sized + IFabricApplicationManagementClient_Impl
{
    fn BeginGetApplicationManifest(
        &self,
        applicationtypename: &::windows_core::PCWSTR,
        applicationtypeversion: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationManifest(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
    fn BeginMoveNextApplicationUpgradeDomain2(
        &self,
        applicationname: *const u16,
        nextupgradedomain: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextApplicationUpgradeDomain2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient2 {}
impl IFabricApplicationManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient2_Vtbl {
        unsafe extern "system" fn BeginGetApplicationManifest<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows_core::PCWSTR,
            applicationtypeversion: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationManifest(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationManifest<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationManifest(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveNextApplicationUpgradeDomain2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            nextupgradedomain: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextApplicationUpgradeDomain2(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute(&nextupgradedomain),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMoveNextApplicationUpgradeDomain2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextApplicationUpgradeDomain2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetApplicationManifest: BeginGetApplicationManifest::<Identity, Impl, OFFSET>,
            EndGetApplicationManifest: EndGetApplicationManifest::<Identity, Impl, OFFSET>,
            BeginMoveNextApplicationUpgradeDomain2: BeginMoveNextApplicationUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextApplicationUpgradeDomain2: EndMoveNextApplicationUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationManagementClient3_Impl:
    Sized + IFabricApplicationManagementClient2_Impl
{
    fn BeginUpdateApplicationUpgrade(
        &self,
        description: *const super::super::FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateApplicationUpgrade(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginRestartDeployedCodePackage(
        &self,
        restartcodepackagedescription : *const super::super:: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartDeployedCodePackage(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn CopyApplicationPackage(
        &self,
        imagestoreconnectionstring: &::windows_core::PCWSTR,
        applicationpackagepath: &::windows_core::PCWSTR,
        applicationpackagepathinimagestore: &::windows_core::PCWSTR,
    ) -> ::windows_core::Result<()>;
    fn RemoveApplicationPackage(
        &self,
        imagestoreconnectionstring: &::windows_core::PCWSTR,
        applicationpackagepathinimagestore: &::windows_core::PCWSTR,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient3 {}
impl IFabricApplicationManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient3_Vtbl {
        unsafe extern "system" fn BeginUpdateApplicationUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateApplicationUpgrade(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateApplicationUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateApplicationUpgrade(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginRestartDeployedCodePackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartcodepackagedescription : *const super::super:: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartDeployedCodePackage(
                ::core::mem::transmute_copy(&restartcodepackagedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRestartDeployedCodePackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRestartDeployedCodePackage(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn CopyApplicationPackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows_core::PCWSTR,
            applicationpackagepath: ::windows_core::PCWSTR,
            applicationpackagepathinimagestore: ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyApplicationPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&applicationpackagepath),
                ::core::mem::transmute(&applicationpackagepathinimagestore),
            )
            .into()
        }
        unsafe extern "system" fn RemoveApplicationPackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows_core::PCWSTR,
            applicationpackagepathinimagestore: ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveApplicationPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&applicationpackagepathinimagestore),
            )
            .into()
        }
        Self {
            base__: IFabricApplicationManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateApplicationUpgrade: BeginUpdateApplicationUpgrade::<Identity, Impl, OFFSET>,
            EndUpdateApplicationUpgrade: EndUpdateApplicationUpgrade::<Identity, Impl, OFFSET>,
            BeginRestartDeployedCodePackage: BeginRestartDeployedCodePackage::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRestartDeployedCodePackage: EndRestartDeployedCodePackage::<Identity, Impl, OFFSET>,
            CopyApplicationPackage: CopyApplicationPackage::<Identity, Impl, OFFSET>,
            RemoveApplicationPackage: RemoveApplicationPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationManagementClient4_Impl:
    Sized + IFabricApplicationManagementClient3_Impl
{
    fn BeginDeployServicePackageToNode(
        &self,
        applicationtypename: &::windows_core::PCWSTR,
        applicationtypeversion: &::windows_core::PCWSTR,
        servicemanifestname: &::windows_core::PCWSTR,
        sharingpolicy: *const super::super::FABRIC_PACKAGE_SHARING_POLICY_LIST,
        nodename: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeployServicePackageToNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient4 {}
impl IFabricApplicationManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient4_Vtbl {
        unsafe extern "system" fn BeginDeployServicePackageToNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows_core::PCWSTR,
            applicationtypeversion: ::windows_core::PCWSTR,
            servicemanifestname: ::windows_core::PCWSTR,
            sharingpolicy: *const super::super::FABRIC_PACKAGE_SHARING_POLICY_LIST,
            nodename: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeployServicePackageToNode(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
                ::core::mem::transmute(&servicemanifestname),
                ::core::mem::transmute_copy(&sharingpolicy),
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeployServicePackageToNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeployServicePackageToNode(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeployServicePackageToNode: BeginDeployServicePackageToNode::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndDeployServicePackageToNode: EndDeployServicePackageToNode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationManagementClient5_Impl:
    Sized + IFabricApplicationManagementClient4_Impl
{
    fn BeginRollbackApplicationUpgrade(
        &self,
        applicationname: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRollbackApplicationUpgrade(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient5 {}
impl IFabricApplicationManagementClient5_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient5_Vtbl {
        unsafe extern "system" fn BeginRollbackApplicationUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRollbackApplicationUpgrade(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRollbackApplicationUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRollbackApplicationUpgrade(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRollbackApplicationUpgrade: BeginRollbackApplicationUpgrade::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRollbackApplicationUpgrade: EndRollbackApplicationUpgrade::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricApplicationManagementClient6_Impl:
    Sized + IFabricApplicationManagementClient5_Impl
{
    fn BeginUpdateApplication(
        &self,
        applicationupdatedescription: *const super::super::FABRIC_APPLICATION_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateApplication(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient6 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricApplicationManagementClient6_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient6_Vtbl {
        unsafe extern "system" fn BeginUpdateApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationupdatedescription : *const super::super:: FABRIC_APPLICATION_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateApplication(
                ::core::mem::transmute_copy(&applicationupdatedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateApplication<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateApplication(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateApplication: BeginUpdateApplication::<Identity, Impl, OFFSET>,
            EndUpdateApplication: EndUpdateApplication::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricApplicationManagementClient7_Impl:
    Sized + IFabricApplicationManagementClient6_Impl
{
    fn BeginDeleteApplication2(
        &self,
        deletedescription: *const super::super::FABRIC_DELETE_APPLICATION_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteApplication2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient7 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricApplicationManagementClient7_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient7_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient7_Vtbl {
        unsafe extern "system" fn BeginDeleteApplication2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            deletedescription: *const super::super::FABRIC_DELETE_APPLICATION_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteApplication2(
                ::core::mem::transmute_copy(&deletedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteApplication2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteApplication2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient6_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeleteApplication2: BeginDeleteApplication2::<Identity, Impl, OFFSET>,
            EndDeleteApplication2: EndDeleteApplication2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricApplicationManagementClient8_Impl:
    Sized + IFabricApplicationManagementClient7_Impl
{
    fn BeginProvisionApplicationType2(
        &self,
        description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionApplicationType2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient8 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricApplicationManagementClient8_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient8_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient8_Vtbl {
        unsafe extern "system" fn BeginProvisionApplicationType2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionApplicationType2(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndProvisionApplicationType2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionApplicationType2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient7_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginProvisionApplicationType2: BeginProvisionApplicationType2::<Identity, Impl, OFFSET>,
            EndProvisionApplicationType2: EndProvisionApplicationType2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient8 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient7 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricApplicationManagementClient9_Impl:
    Sized + IFabricApplicationManagementClient8_Impl
{
    fn BeginUnprovisionApplicationType2(
        &self,
        description: *const super::super::FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnprovisionApplicationType2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricApplicationManagementClient9 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricApplicationManagementClient9_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient9_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient9_Vtbl {
        unsafe extern "system" fn BeginUnprovisionApplicationType2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnprovisionApplicationType2(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnprovisionApplicationType2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnprovisionApplicationType2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient8_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUnprovisionApplicationType2: BeginUnprovisionApplicationType2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUnprovisionApplicationType2: EndUnprovisionApplicationType2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient9 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricApplicationManagementClient8 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationUpgradeProgressResult_Impl: Sized {
    fn get_ApplicationName(&self) -> *mut u16;
    fn get_ApplicationTypeName(&self) -> ::windows_core::PCWSTR;
    fn get_TargetApplicationTypeVersion(&self) -> ::windows_core::PCWSTR;
    fn get_UpgradeState(&self) -> super::super::FABRIC_APPLICATION_UPGRADE_STATE;
    fn GetUpgradeDomains(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows_core::Result<()>;
    fn GetChangedUpgradeDomains(
        &self,
        previousprogress: ::core::option::Option<&IFabricApplicationUpgradeProgressResult>,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricApplicationUpgradeProgressResult {}
impl IFabricApplicationUpgradeProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationUpgradeProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationUpgradeProgressResult_Vtbl {
        unsafe extern "system" fn get_ApplicationName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut u16 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationName()
        }
        unsafe extern "system" fn get_ApplicationTypeName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationTypeName()
        }
        unsafe extern "system" fn get_TargetApplicationTypeVersion<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TargetApplicationTypeVersion()
        }
        unsafe extern "system" fn get_UpgradeState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_APPLICATION_UPGRADE_STATE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeState()
        }
        unsafe extern "system" fn GetUpgradeDomains<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpgradeDomains(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        unsafe extern "system" fn GetChangedUpgradeDomains<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            previousprogress: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChangedUpgradeDomains(
                ::windows_core::from_raw_borrowed(&previousprogress),
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationName: get_ApplicationName::<Identity, Impl, OFFSET>,
            get_ApplicationTypeName: get_ApplicationTypeName::<Identity, Impl, OFFSET>,
            get_TargetApplicationTypeVersion: get_TargetApplicationTypeVersion::<
                Identity,
                Impl,
                OFFSET,
            >,
            get_UpgradeState: get_UpgradeState::<Identity, Impl, OFFSET>,
            GetUpgradeDomains: GetUpgradeDomains::<Identity, Impl, OFFSET>,
            GetChangedUpgradeDomains: GetChangedUpgradeDomains::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationUpgradeProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationUpgradeProgressResult2_Impl:
    Sized + IFabricApplicationUpgradeProgressResult_Impl
{
    fn get_RollingUpgradeMode(&self) -> super::super::FABRIC_ROLLING_UPGRADE_MODE;
    fn get_NextUpgradeDomain(&self) -> ::windows_core::PCWSTR;
}
impl ::windows_core::RuntimeName for IFabricApplicationUpgradeProgressResult2 {}
impl IFabricApplicationUpgradeProgressResult2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationUpgradeProgressResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationUpgradeProgressResult2_Vtbl {
        unsafe extern "system" fn get_RollingUpgradeMode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ROLLING_UPGRADE_MODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_RollingUpgradeMode()
        }
        unsafe extern "system" fn get_NextUpgradeDomain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NextUpgradeDomain()
        }
        Self {
            base__: IFabricApplicationUpgradeProgressResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_RollingUpgradeMode: get_RollingUpgradeMode::<Identity, Impl, OFFSET>,
            get_NextUpgradeDomain: get_NextUpgradeDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationUpgradeProgressResult2 as ::windows_core::ComInterface>::IID
            || iid
                == &<IFabricApplicationUpgradeProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricApplicationUpgradeProgressResult3_Impl:
    Sized + IFabricApplicationUpgradeProgressResult2_Impl
{
    fn get_UpgradeProgress(&self) -> *mut super::super::FABRIC_APPLICATION_UPGRADE_PROGRESS;
}
impl ::windows_core::RuntimeName for IFabricApplicationUpgradeProgressResult3 {}
impl IFabricApplicationUpgradeProgressResult3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationUpgradeProgressResult3_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationUpgradeProgressResult3_Vtbl {
        unsafe extern "system" fn get_UpgradeProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_UPGRADE_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeProgress()
        }
        Self {
            base__: IFabricApplicationUpgradeProgressResult2_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_UpgradeProgress: get_UpgradeProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricApplicationUpgradeProgressResult3 as ::windows_core::ComInterface>::IID
            || iid
                == &<IFabricApplicationUpgradeProgressResult as ::windows_core::ComInterface>::IID
            || iid
                == &<IFabricApplicationUpgradeProgressResult2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricChaosDescriptionResult_Impl: Sized {
    fn get_ChaosDescriptionResult(&self) -> *mut super::super::FABRIC_CHAOS_DESCRIPTION;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricChaosDescriptionResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricChaosDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosDescriptionResult_Vtbl {
        unsafe extern "system" fn get_ChaosDescriptionResult<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosDescriptionResult()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosDescriptionResult: get_ChaosDescriptionResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricChaosDescriptionResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricChaosEventsSegmentResult_Impl: Sized {
    fn get_ChaosEventsSegmentResult(&self) -> *mut super::super::FABRIC_CHAOS_EVENTS_SEGMENT;
}
impl ::windows_core::RuntimeName for IFabricChaosEventsSegmentResult {}
impl IFabricChaosEventsSegmentResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosEventsSegmentResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosEventsSegmentResult_Vtbl {
        unsafe extern "system" fn get_ChaosEventsSegmentResult<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosEventsSegmentResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_EVENTS_SEGMENT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosEventsSegmentResult()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosEventsSegmentResult: get_ChaosEventsSegmentResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricChaosEventsSegmentResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricChaosReportResult_Impl: Sized {
    fn get_ChaosReportResult(&self) -> *mut super::super::FABRIC_CHAOS_REPORT;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricChaosReportResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricChaosReportResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosReportResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosReportResult_Vtbl {
        unsafe extern "system" fn get_ChaosReportResult<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosReportResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_REPORT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosReportResult()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosReportResult: get_ChaosReportResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricChaosReportResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricChaosScheduleDescriptionResult_Impl: Sized {
    fn get_ChaosScheduleDescriptionResult(
        &self,
    ) -> *mut super::super::FABRIC_CHAOS_SCHEDULE_DESCRIPTION;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricChaosScheduleDescriptionResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricChaosScheduleDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosScheduleDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosScheduleDescriptionResult_Vtbl {
        unsafe extern "system" fn get_ChaosScheduleDescriptionResult<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosScheduleDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_SCHEDULE_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosScheduleDescriptionResult()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosScheduleDescriptionResult: get_ChaosScheduleDescriptionResult::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricChaosScheduleDescriptionResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClientConnectionEventHandler_Impl: Sized {
    fn OnConnected(
        &self,
        __midl__ifabricclientconnectioneventhandler0000: ::core::option::Option<
            &IFabricGatewayInformationResult,
        >,
    ) -> ::windows_core::Result<()>;
    fn OnDisconnected(
        &self,
        __midl__ifabricclientconnectioneventhandler0001: ::core::option::Option<
            &IFabricGatewayInformationResult,
        >,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClientConnectionEventHandler {}
impl IFabricClientConnectionEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientConnectionEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricClientConnectionEventHandler_Vtbl {
        unsafe extern "system" fn OnConnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientConnectionEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            __midl__ifabricclientconnectioneventhandler0000: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnected(::windows_core::from_raw_borrowed(
                &__midl__ifabricclientconnectioneventhandler0000,
            ))
            .into()
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientConnectionEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            __midl__ifabricclientconnectioneventhandler0001: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnected(::windows_core::from_raw_borrowed(
                &__midl__ifabricclientconnectioneventhandler0001,
            ))
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClientConnectionEventHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClientConnectionEventHandler2_Impl:
    Sized + IFabricClientConnectionEventHandler_Impl
{
    fn OnClaimsRetrieval(
        &self,
        metadata: *const super::super::FABRIC_CLAIMS_RETRIEVAL_METADATA,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
}
impl ::windows_core::RuntimeName for IFabricClientConnectionEventHandler2 {}
impl IFabricClientConnectionEventHandler2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientConnectionEventHandler2_Impl,
        const OFFSET: isize,
    >() -> IFabricClientConnectionEventHandler2_Vtbl {
        unsafe extern "system" fn OnClaimsRetrieval<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientConnectionEventHandler2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            metadata: *const super::super::FABRIC_CLAIMS_RETRIEVAL_METADATA,
            token: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnClaimsRetrieval(::core::mem::transmute_copy(&metadata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClientConnectionEventHandler_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnClaimsRetrieval: OnClaimsRetrieval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClientConnectionEventHandler2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClientConnectionEventHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClientSettings_Impl: Sized {
    fn SetSecurityCredentials(
        &self,
        securitycredentials: *const super::super::FABRIC_SECURITY_CREDENTIALS,
    ) -> ::windows_core::Result<()>;
    fn SetKeepAlive(&self, keepaliveintervalinseconds: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClientSettings {}
impl IFabricClientSettings_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientSettings_Impl,
        const OFFSET: isize,
    >() -> IFabricClientSettings_Vtbl {
        unsafe extern "system" fn SetSecurityCredentials<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            securitycredentials: *const super::super::FABRIC_SECURITY_CREDENTIALS,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityCredentials(::core::mem::transmute_copy(&securitycredentials))
                .into()
        }
        unsafe extern "system" fn SetKeepAlive<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            keepaliveintervalinseconds: u32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeepAlive(::core::mem::transmute_copy(&keepaliveintervalinseconds))
                .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSecurityCredentials: SetSecurityCredentials::<Identity, Impl, OFFSET>,
            SetKeepAlive: SetKeepAlive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClientSettings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClientSettings2_Impl: Sized + IFabricClientSettings_Impl {
    fn GetSettings(&self) -> ::windows_core::Result<IFabricClientSettingsResult>;
    fn SetSettings(
        &self,
        fabricclientsettings: *const super::super::FABRIC_CLIENT_SETTINGS,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClientSettings2 {}
impl IFabricClientSettings2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientSettings2_Impl,
        const OFFSET: isize,
    >() -> IFabricClientSettings2_Vtbl {
        unsafe extern "system" fn GetSettings<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettings<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            fabricclientsettings: *const super::super::FABRIC_CLIENT_SETTINGS,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSettings(::core::mem::transmute_copy(&fabricclientsettings))
                .into()
        }
        Self {
            base__: IFabricClientSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSettings: GetSettings::<Identity, Impl, OFFSET>,
            SetSettings: SetSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClientSettings2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClientSettings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClientSettingsResult_Impl: Sized {
    fn get_Settings(&self) -> *mut super::super::FABRIC_CLIENT_SETTINGS;
}
impl ::windows_core::RuntimeName for IFabricClientSettingsResult {}
impl IFabricClientSettingsResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientSettingsResult_Impl,
        const OFFSET: isize,
    >() -> IFabricClientSettingsResult_Vtbl {
        unsafe extern "system" fn get_Settings<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettingsResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLIENT_SETTINGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Settings()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Settings: get_Settings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClientSettingsResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClusterHealthResult_Impl: Sized {
    fn get_ClusterHealth(&self) -> *mut super::super::FABRIC_CLUSTER_HEALTH;
}
impl ::windows_core::RuntimeName for IFabricClusterHealthResult {}
impl IFabricClusterHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterHealthResult_Vtbl {
        unsafe extern "system" fn get_ClusterHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLUSTER_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ClusterHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ClusterHealth: get_ClusterHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClusterManagementClient_Impl: Sized {
    fn BeginNodeStateRemoved(
        &self,
        nodename: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndNodeStateRemoved(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginRecoverPartitions(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverPartitions(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClusterManagementClient {}
impl IFabricClusterManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient_Vtbl {
        unsafe extern "system" fn BeginNodeStateRemoved<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginNodeStateRemoved(
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndNodeStateRemoved<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndNodeStateRemoved(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginRecoverPartitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverPartitions(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRecoverPartitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverPartitions(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginNodeStateRemoved: BeginNodeStateRemoved::<Identity, Impl, OFFSET>,
            EndNodeStateRemoved: EndNodeStateRemoved::<Identity, Impl, OFFSET>,
            BeginRecoverPartitions: BeginRecoverPartitions::<Identity, Impl, OFFSET>,
            EndRecoverPartitions: EndRecoverPartitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricClusterManagementClient10_Impl:
    Sized + IFabricClusterManagementClient9_Impl
{
    fn BeginGetClusterConfiguration2(
        &self,
        apiversion: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterConfiguration2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricClusterManagementClient10 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricClusterManagementClient10_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient10_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient10_Vtbl {
        unsafe extern "system" fn BeginGetClusterConfiguration2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            apiversion: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterConfiguration2(
                ::core::mem::transmute(&apiversion),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterConfiguration2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterConfiguration2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClusterManagementClient9_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterConfiguration2: BeginGetClusterConfiguration2::<Identity, Impl, OFFSET>,
            EndGetClusterConfiguration2: EndGetClusterConfiguration2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient10 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient8 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient9 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClusterManagementClient2_Impl:
    Sized + IFabricClusterManagementClient_Impl
{
    fn BeginDeactivateNode(
        &self,
        nodename: &::windows_core::PCWSTR,
        intent: super::super::FABRIC_NODE_DEACTIVATION_INTENT,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeactivateNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginActivateNode(
        &self,
        nodename: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndActivateNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginProvisionFabric(
        &self,
        codefilepath: &::windows_core::PCWSTR,
        clustermanifestfilepath: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionFabric(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginUpgradeFabric(
        &self,
        upgradedescription: *const super::super::FABRIC_UPGRADE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpgradeFabric(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetFabricUpgradeProgress(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetFabricUpgradeProgress(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricUpgradeProgressResult2>;
    fn BeginMoveNextFabricUpgradeDomain(
        &self,
        progress: ::core::option::Option<&IFabricUpgradeProgressResult2>,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextFabricUpgradeDomain(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginMoveNextFabricUpgradeDomain2(
        &self,
        nextupgradedomain: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextFabricUpgradeDomain2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginUnprovisionFabric(
        &self,
        codeversion: &::windows_core::PCWSTR,
        configversion: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnprovisionFabric(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetClusterManifest(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterManifest(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
    fn BeginRecoverPartition(
        &self,
        partitionid: &::windows_core::GUID,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverPartition(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginRecoverServicePartitions(
        &self,
        servicename: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverServicePartitions(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginRecoverSystemPartitions(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverSystemPartitions(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClusterManagementClient2 {}
impl IFabricClusterManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient2_Vtbl {
        unsafe extern "system" fn BeginDeactivateNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows_core::PCWSTR,
            intent: super::super::FABRIC_NODE_DEACTIVATION_INTENT,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeactivateNode(
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&intent),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeactivateNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeactivateNode(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginActivateNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginActivateNode(
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndActivateNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndActivateNode(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginProvisionFabric<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codefilepath: ::windows_core::PCWSTR,
            clustermanifestfilepath: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionFabric(
                ::core::mem::transmute(&codefilepath),
                ::core::mem::transmute(&clustermanifestfilepath),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndProvisionFabric<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionFabric(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginUpgradeFabric<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            upgradedescription: *const super::super::FABRIC_UPGRADE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpgradeFabric(
                ::core::mem::transmute_copy(&upgradedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpgradeFabric<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpgradeFabric(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetFabricUpgradeProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetFabricUpgradeProgress(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetFabricUpgradeProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetFabricUpgradeProgress(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveNextFabricUpgradeDomain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            progress: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextFabricUpgradeDomain(
                ::windows_core::from_raw_borrowed(&progress),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMoveNextFabricUpgradeDomain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextFabricUpgradeDomain(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginMoveNextFabricUpgradeDomain2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nextupgradedomain: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextFabricUpgradeDomain2(
                ::core::mem::transmute(&nextupgradedomain),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMoveNextFabricUpgradeDomain2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextFabricUpgradeDomain2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginUnprovisionFabric<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codeversion: ::windows_core::PCWSTR,
            configversion: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnprovisionFabric(
                ::core::mem::transmute(&codeversion),
                ::core::mem::transmute(&configversion),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnprovisionFabric<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnprovisionFabric(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetClusterManifest<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterManifest(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterManifest<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterManifest(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRecoverPartition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverPartition(
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRecoverPartition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverPartition(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginRecoverServicePartitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverServicePartitions(
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRecoverServicePartitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverServicePartitions(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginRecoverSystemPartitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverSystemPartitions(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRecoverSystemPartitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverSystemPartitions(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeactivateNode: BeginDeactivateNode::<Identity, Impl, OFFSET>,
            EndDeactivateNode: EndDeactivateNode::<Identity, Impl, OFFSET>,
            BeginActivateNode: BeginActivateNode::<Identity, Impl, OFFSET>,
            EndActivateNode: EndActivateNode::<Identity, Impl, OFFSET>,
            BeginProvisionFabric: BeginProvisionFabric::<Identity, Impl, OFFSET>,
            EndProvisionFabric: EndProvisionFabric::<Identity, Impl, OFFSET>,
            BeginUpgradeFabric: BeginUpgradeFabric::<Identity, Impl, OFFSET>,
            EndUpgradeFabric: EndUpgradeFabric::<Identity, Impl, OFFSET>,
            BeginGetFabricUpgradeProgress: BeginGetFabricUpgradeProgress::<Identity, Impl, OFFSET>,
            EndGetFabricUpgradeProgress: EndGetFabricUpgradeProgress::<Identity, Impl, OFFSET>,
            BeginMoveNextFabricUpgradeDomain: BeginMoveNextFabricUpgradeDomain::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextFabricUpgradeDomain: EndMoveNextFabricUpgradeDomain::<Identity, Impl, OFFSET>,
            BeginMoveNextFabricUpgradeDomain2: BeginMoveNextFabricUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextFabricUpgradeDomain2: EndMoveNextFabricUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginUnprovisionFabric: BeginUnprovisionFabric::<Identity, Impl, OFFSET>,
            EndUnprovisionFabric: EndUnprovisionFabric::<Identity, Impl, OFFSET>,
            BeginGetClusterManifest: BeginGetClusterManifest::<Identity, Impl, OFFSET>,
            EndGetClusterManifest: EndGetClusterManifest::<Identity, Impl, OFFSET>,
            BeginRecoverPartition: BeginRecoverPartition::<Identity, Impl, OFFSET>,
            EndRecoverPartition: EndRecoverPartition::<Identity, Impl, OFFSET>,
            BeginRecoverServicePartitions: BeginRecoverServicePartitions::<Identity, Impl, OFFSET>,
            EndRecoverServicePartitions: EndRecoverServicePartitions::<Identity, Impl, OFFSET>,
            BeginRecoverSystemPartitions: BeginRecoverSystemPartitions::<Identity, Impl, OFFSET>,
            EndRecoverSystemPartitions: EndRecoverSystemPartitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClusterManagementClient3_Impl:
    Sized + IFabricClusterManagementClient2_Impl
{
    fn BeginUpdateFabricUpgrade(
        &self,
        description: *const super::super::FABRIC_UPGRADE_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateFabricUpgrade(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginStopNode(
        &self,
        stopnodedescription: *const super::super::FABRIC_STOP_NODE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStopNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginRestartNode(
        &self,
        restartnodedescription: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginStartNode(
        &self,
        startnodedescription: *const super::super::FABRIC_START_NODE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn CopyClusterPackage(
        &self,
        imagestoreconnectionstring: &::windows_core::PCWSTR,
        clustermanifestpath: &::windows_core::PCWSTR,
        clustermanifestpathinimagestore: &::windows_core::PCWSTR,
        codepackagepath: &::windows_core::PCWSTR,
        codepackagepathinimagestore: &::windows_core::PCWSTR,
    ) -> ::windows_core::Result<()>;
    fn RemoveClusterPackage(
        &self,
        imagestoreconnectionstring: &::windows_core::PCWSTR,
        clustermanifestpathinimagestore: &::windows_core::PCWSTR,
        codepackagepathinimagestore: &::windows_core::PCWSTR,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClusterManagementClient3 {}
impl IFabricClusterManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient3_Vtbl {
        unsafe extern "system" fn BeginUpdateFabricUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_UPGRADE_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateFabricUpgrade(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateFabricUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateFabricUpgrade(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginStopNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            stopnodedescription: *const super::super::FABRIC_STOP_NODE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStopNode(
                ::core::mem::transmute_copy(&stopnodedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStopNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStopNode(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginRestartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartnodedescription: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartNode(
                ::core::mem::transmute_copy(&restartnodedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRestartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRestartNode(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginStartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            startnodedescription: *const super::super::FABRIC_START_NODE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartNode(
                ::core::mem::transmute_copy(&startnodedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartNode(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn CopyClusterPackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows_core::PCWSTR,
            clustermanifestpath: ::windows_core::PCWSTR,
            clustermanifestpathinimagestore: ::windows_core::PCWSTR,
            codepackagepath: ::windows_core::PCWSTR,
            codepackagepathinimagestore: ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyClusterPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&clustermanifestpath),
                ::core::mem::transmute(&clustermanifestpathinimagestore),
                ::core::mem::transmute(&codepackagepath),
                ::core::mem::transmute(&codepackagepathinimagestore),
            )
            .into()
        }
        unsafe extern "system" fn RemoveClusterPackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows_core::PCWSTR,
            clustermanifestpathinimagestore: ::windows_core::PCWSTR,
            codepackagepathinimagestore: ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveClusterPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&clustermanifestpathinimagestore),
                ::core::mem::transmute(&codepackagepathinimagestore),
            )
            .into()
        }
        Self {
            base__: IFabricClusterManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateFabricUpgrade: BeginUpdateFabricUpgrade::<Identity, Impl, OFFSET>,
            EndUpdateFabricUpgrade: EndUpdateFabricUpgrade::<Identity, Impl, OFFSET>,
            BeginStopNode: BeginStopNode::<Identity, Impl, OFFSET>,
            EndStopNode: EndStopNode::<Identity, Impl, OFFSET>,
            BeginRestartNode: BeginRestartNode::<Identity, Impl, OFFSET>,
            EndRestartNode: EndRestartNode::<Identity, Impl, OFFSET>,
            BeginStartNode: BeginStartNode::<Identity, Impl, OFFSET>,
            EndStartNode: EndStartNode::<Identity, Impl, OFFSET>,
            CopyClusterPackage: CopyClusterPackage::<Identity, Impl, OFFSET>,
            RemoveClusterPackage: RemoveClusterPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClusterManagementClient4_Impl:
    Sized + IFabricClusterManagementClient3_Impl
{
    fn BeginRollbackFabricUpgrade(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRollbackFabricUpgrade(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClusterManagementClient4 {}
impl IFabricClusterManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient4_Vtbl {
        unsafe extern "system" fn BeginRollbackFabricUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRollbackFabricUpgrade(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRollbackFabricUpgrade<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRollbackFabricUpgrade(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRollbackFabricUpgrade: BeginRollbackFabricUpgrade::<Identity, Impl, OFFSET>,
            EndRollbackFabricUpgrade: EndRollbackFabricUpgrade::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricClusterManagementClient5_Impl:
    Sized + IFabricClusterManagementClient4_Impl
{
    fn BeginResetPartitionLoad(
        &self,
        partitionid: &::windows_core::GUID,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndResetPartitionLoad(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricClusterManagementClient5 {}
impl IFabricClusterManagementClient5_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient5_Vtbl {
        unsafe extern "system" fn BeginResetPartitionLoad<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginResetPartitionLoad(
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndResetPartitionLoad<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndResetPartitionLoad(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginResetPartitionLoad: BeginResetPartitionLoad::<Identity, Impl, OFFSET>,
            EndResetPartitionLoad: EndResetPartitionLoad::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricClusterManagementClient6_Impl:
    Sized + IFabricClusterManagementClient5_Impl
{
    fn BeginToggleVerboseServicePlacementHealthReporting(
        &self,
        enabled: ::windows::Win32::Foundation::BOOLEAN,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndToggleVerboseServicePlacementHealthReporting(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricClusterManagementClient6 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricClusterManagementClient6_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient6_Vtbl {
        unsafe extern "system" fn BeginToggleVerboseServicePlacementHealthReporting<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            enabled: ::windows::Win32::Foundation::BOOLEAN,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginToggleVerboseServicePlacementHealthReporting(
                ::core::mem::transmute_copy(&enabled),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndToggleVerboseServicePlacementHealthReporting<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndToggleVerboseServicePlacementHealthReporting(::windows_core::from_raw_borrowed(
                &context,
            ))
            .into()
        }
        Self {
            base__: IFabricClusterManagementClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginToggleVerboseServicePlacementHealthReporting:
                BeginToggleVerboseServicePlacementHealthReporting::<Identity, Impl, OFFSET>,
            EndToggleVerboseServicePlacementHealthReporting:
                EndToggleVerboseServicePlacementHealthReporting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricClusterManagementClient7_Impl:
    Sized + IFabricClusterManagementClient6_Impl
{
    fn BeginUpgradeConfiguration(
        &self,
        startupgradedescription: *const super::super::FABRIC_START_UPGRADE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpgradeConfiguration(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetClusterConfigurationUpgradeStatus(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterConfigurationUpgradeStatus(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricOrchestrationUpgradeStatusResult>;
    fn BeginGetClusterConfiguration(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterConfiguration(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
    fn BeginGetUpgradesPendingApproval(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetUpgradesPendingApproval(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginStartApprovedUpgrades(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartApprovedUpgrades(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricClusterManagementClient7 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricClusterManagementClient7_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient7_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient7_Vtbl {
        unsafe extern "system" fn BeginUpgradeConfiguration<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            startupgradedescription: *const super::super::FABRIC_START_UPGRADE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpgradeConfiguration(
                ::core::mem::transmute_copy(&startupgradedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpgradeConfiguration<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpgradeConfiguration(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetClusterConfigurationUpgradeStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterConfigurationUpgradeStatus(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterConfigurationUpgradeStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterConfigurationUpgradeStatus(::windows_core::from_raw_borrowed(
                &context,
            )) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetClusterConfiguration<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterConfiguration(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterConfiguration<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterConfiguration(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetUpgradesPendingApproval<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetUpgradesPendingApproval(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetUpgradesPendingApproval<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndGetUpgradesPendingApproval(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginStartApprovedUpgrades<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartApprovedUpgrades(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartApprovedUpgrades<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartApprovedUpgrades(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient6_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpgradeConfiguration: BeginUpgradeConfiguration::<Identity, Impl, OFFSET>,
            EndUpgradeConfiguration: EndUpgradeConfiguration::<Identity, Impl, OFFSET>,
            BeginGetClusterConfigurationUpgradeStatus: BeginGetClusterConfigurationUpgradeStatus::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetClusterConfigurationUpgradeStatus: EndGetClusterConfigurationUpgradeStatus::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetClusterConfiguration: BeginGetClusterConfiguration::<Identity, Impl, OFFSET>,
            EndGetClusterConfiguration: EndGetClusterConfiguration::<Identity, Impl, OFFSET>,
            BeginGetUpgradesPendingApproval: BeginGetUpgradesPendingApproval::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetUpgradesPendingApproval: EndGetUpgradesPendingApproval::<Identity, Impl, OFFSET>,
            BeginStartApprovedUpgrades: BeginStartApprovedUpgrades::<Identity, Impl, OFFSET>,
            EndStartApprovedUpgrades: EndStartApprovedUpgrades::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricClusterManagementClient8_Impl:
    Sized + IFabricClusterManagementClient7_Impl
{
    fn BeginGetClusterManifest2(
        &self,
        querydescription: *const super::super::FABRIC_CLUSTER_MANIFEST_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterManifest2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricClusterManagementClient8 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricClusterManagementClient8_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient8_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient8_Vtbl {
        unsafe extern "system" fn BeginGetClusterManifest2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_CLUSTER_MANIFEST_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterManifest2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterManifest2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterManifest2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClusterManagementClient7_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterManifest2: BeginGetClusterManifest2::<Identity, Impl, OFFSET>,
            EndGetClusterManifest2: EndGetClusterManifest2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient8 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient7 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricClusterManagementClient9_Impl:
    Sized + IFabricClusterManagementClient8_Impl
{
    fn BeginGetUpgradeOrchestrationServiceState(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetUpgradeOrchestrationServiceState(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
    fn BeginSetUpgradeOrchestrationServiceState(
        &self,
        state: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndSetUpgradeOrchestrationServiceState(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricUpgradeOrchestrationServiceStateResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricClusterManagementClient9 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricClusterManagementClient9_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient9_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient9_Vtbl {
        unsafe extern "system" fn BeginGetUpgradeOrchestrationServiceState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetUpgradeOrchestrationServiceState(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetUpgradeOrchestrationServiceState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndGetUpgradeOrchestrationServiceState(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSetUpgradeOrchestrationServiceState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            state: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSetUpgradeOrchestrationServiceState(
                ::core::mem::transmute(&state),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSetUpgradeOrchestrationServiceState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndSetUpgradeOrchestrationServiceState(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClusterManagementClient8_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetUpgradeOrchestrationServiceState: BeginGetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetUpgradeOrchestrationServiceState: EndGetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginSetUpgradeOrchestrationServiceState: BeginSetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndSetUpgradeOrchestrationServiceState: EndSetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient9 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricClusterManagementClient8 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricDeployedApplicationHealthResult_Impl: Sized {
    fn get_DeployedApplicationHealth(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_HEALTH;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricDeployedApplicationHealthResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricDeployedApplicationHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricDeployedApplicationHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricDeployedApplicationHealthResult_Vtbl {
        unsafe extern "system" fn get_DeployedApplicationHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDeployedApplicationHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedApplicationHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedApplicationHealth: get_DeployedApplicationHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricDeployedApplicationHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricDeployedServicePackageHealthResult_Impl: Sized {
    fn get_DeployedServicePackageHealth(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricDeployedServicePackageHealthResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricDeployedServicePackageHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricDeployedServicePackageHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricDeployedServicePackageHealthResult_Vtbl {
        unsafe extern "system" fn get_DeployedServicePackageHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDeployedServicePackageHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedServicePackageHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedServicePackageHealth: get_DeployedServicePackageHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricDeployedServicePackageHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricFaultManagementClient_Impl: Sized {
    fn BeginRestartNode(
        &self,
        description: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricRestartNodeResult>;
    fn BeginStartNode(
        &self,
        description: *const super::super::FABRIC_START_NODE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStartNodeResult>;
    fn BeginStopNode(
        &self,
        description: *const super::super::FABRIC_STOP_NODE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStopNode(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStopNodeResult>;
    fn BeginRestartDeployedCodePackage(
        &self,
        description: *const super::super::FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartDeployedCodePackage(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricRestartDeployedCodePackageResult>;
    fn BeginMovePrimary(
        &self,
        description: *const super::super::FABRIC_MOVE_PRIMARY_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndMovePrimary(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricMovePrimaryResult>;
    fn BeginMoveSecondary(
        &self,
        description: *const super::super::FABRIC_MOVE_SECONDARY_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveSecondary(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricMoveSecondaryResult>;
}
impl ::windows_core::RuntimeName for IFabricFaultManagementClient {}
impl IFabricFaultManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricFaultManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricFaultManagementClient_Vtbl {
        unsafe extern "system" fn BeginRestartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartNode(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRestartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRestartNode(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_START_NODE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartNode(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndStartNode(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStopNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_STOP_NODE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStopNode(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStopNode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndStopNode(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRestartDeployedCodePackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartDeployedCodePackage(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRestartDeployedCodePackage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRestartDeployedCodePackage(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMovePrimary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_MOVE_PRIMARY_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMovePrimary(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMovePrimary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndMovePrimary(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveSecondary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_MOVE_SECONDARY_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveSecondary(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMoveSecondary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndMoveSecondary(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginRestartNode: BeginRestartNode::<Identity, Impl, OFFSET>,
            EndRestartNode: EndRestartNode::<Identity, Impl, OFFSET>,
            BeginStartNode: BeginStartNode::<Identity, Impl, OFFSET>,
            EndStartNode: EndStartNode::<Identity, Impl, OFFSET>,
            BeginStopNode: BeginStopNode::<Identity, Impl, OFFSET>,
            EndStopNode: EndStopNode::<Identity, Impl, OFFSET>,
            BeginRestartDeployedCodePackage: BeginRestartDeployedCodePackage::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRestartDeployedCodePackage: EndRestartDeployedCodePackage::<Identity, Impl, OFFSET>,
            BeginMovePrimary: BeginMovePrimary::<Identity, Impl, OFFSET>,
            EndMovePrimary: EndMovePrimary::<Identity, Impl, OFFSET>,
            BeginMoveSecondary: BeginMoveSecondary::<Identity, Impl, OFFSET>,
            EndMoveSecondary: EndMoveSecondary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricFaultManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGatewayInformationResult_Impl: Sized {
    fn get_GatewayInformation(&self) -> *mut super::super::FABRIC_GATEWAY_INFORMATION;
}
impl ::windows_core::RuntimeName for IFabricGatewayInformationResult {}
impl IFabricGatewayInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGatewayInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGatewayInformationResult_Vtbl {
        unsafe extern "system" fn get_GatewayInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGatewayInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_GATEWAY_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_GatewayInformation()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_GatewayInformation: get_GatewayInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGatewayInformationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetApplicationListResult_Impl: Sized {
    fn get_ApplicationList(&self) -> *mut super::super::FABRIC_APPLICATION_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetApplicationListResult {}
impl IFabricGetApplicationListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationList: get_ApplicationList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetApplicationListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetApplicationListResult2_Impl:
    Sized + IFabricGetApplicationListResult_Impl
{
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetApplicationListResult2 {}
impl IFabricGetApplicationListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetApplicationListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetApplicationListResult2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricGetApplicationListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetApplicationLoadInformationResult_Impl: Sized {
    fn get_ApplicationLoadInformation(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_LOAD_INFORMATION;
}
impl ::windows_core::RuntimeName for IFabricGetApplicationLoadInformationResult {}
impl IFabricGetApplicationLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_ApplicationLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationLoadInformation()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationLoadInformation: get_ApplicationLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetApplicationLoadInformationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetApplicationNameResult_Impl: Sized {
    fn get_ApplicationName(&self) -> *mut super::super::FABRIC_APPLICATION_NAME_QUERY_RESULT;
}
impl ::windows_core::RuntimeName for IFabricGetApplicationNameResult {}
impl IFabricGetApplicationNameResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationNameResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationNameResult_Vtbl {
        unsafe extern "system" fn get_ApplicationName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationNameResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_NAME_QUERY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationName()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationName: get_ApplicationName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetApplicationNameResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetApplicationNetworkListResult_Impl: Sized {
    fn get_ApplicationNetworkList(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_NETWORK_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetApplicationNetworkListResult {}
impl IFabricGetApplicationNetworkListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationNetworkListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationNetworkListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_NETWORK_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationNetworkList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationNetworkList: get_ApplicationNetworkList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetApplicationNetworkListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetApplicationTypeListResult_Impl: Sized {
    fn get_ApplicationTypeList(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetApplicationTypeListResult {}
impl IFabricGetApplicationTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationTypeListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationTypeList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationTypeList: get_ApplicationTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetApplicationTypeListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetApplicationTypePagedListResult_Impl: Sized {
    fn get_ApplicationTypePagedList(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetApplicationTypePagedListResult {}
impl IFabricGetApplicationTypePagedListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationTypePagedListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationTypePagedListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationTypePagedList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationTypePagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationTypePagedList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationTypePagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationTypePagedList: get_ApplicationTypePagedList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetApplicationTypePagedListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetClusterHealthChunkResult_Impl: Sized {
    fn get_ClusterHealthChunk(&self) -> *mut super::super::FABRIC_CLUSTER_HEALTH_CHUNK;
}
impl ::windows_core::RuntimeName for IFabricGetClusterHealthChunkResult {}
impl IFabricGetClusterHealthChunkResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetClusterHealthChunkResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetClusterHealthChunkResult_Vtbl {
        unsafe extern "system" fn get_ClusterHealthChunk<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetClusterHealthChunkResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLUSTER_HEALTH_CHUNK {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ClusterHealthChunk()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ClusterHealthChunk: get_ClusterHealthChunk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetClusterHealthChunkResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetClusterLoadInformationResult_Impl: Sized {
    fn get_ClusterLoadInformation(&self) -> *mut super::super::FABRIC_CLUSTER_LOAD_INFORMATION;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetClusterLoadInformationResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetClusterLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetClusterLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetClusterLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_ClusterLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetClusterLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLUSTER_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ClusterLoadInformation()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ClusterLoadInformation: get_ClusterLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetClusterLoadInformationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedApplicationListResult_Impl: Sized {
    fn get_DeployedApplicationList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedApplicationListResult {}
impl IFabricGetDeployedApplicationListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedApplicationListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedApplicationListResult_Vtbl {
        unsafe extern "system" fn get_DeployedApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedApplicationList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedApplicationList: get_DeployedApplicationList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedApplicationListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedApplicationPagedListResult_Impl: Sized {
    fn get_DeployedApplicationPagedList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedApplicationPagedListResult {}
impl IFabricGetDeployedApplicationPagedListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedApplicationPagedListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedApplicationPagedListResult_Vtbl {
        unsafe extern "system" fn get_DeployedApplicationPagedList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedApplicationPagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedApplicationPagedList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedApplicationPagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedApplicationPagedList: get_DeployedApplicationPagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedApplicationPagedListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetDeployedCodePackageListResult_Impl: Sized {
    fn get_DeployedCodePackageList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_LIST;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetDeployedCodePackageListResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetDeployedCodePackageListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedCodePackageListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedCodePackageListResult_Vtbl {
        unsafe extern "system" fn get_DeployedCodePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedCodePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedCodePackageList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedCodePackageList: get_DeployedCodePackageList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedCodePackageListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedNetworkCodePackageListResult_Impl: Sized {
    fn get_DeployedNetworkCodePackageList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedNetworkCodePackageListResult {}
impl IFabricGetDeployedNetworkCodePackageListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedNetworkCodePackageListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedNetworkCodePackageListResult_Vtbl {
        unsafe extern "system" fn get_DeployedNetworkCodePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkCodePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedNetworkCodePackageList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkCodePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedNetworkCodePackageList: get_DeployedNetworkCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == & < IFabricGetDeployedNetworkCodePackageListResult < > as::windows_core::ComInterface >::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedNetworkListResult_Impl: Sized {
    fn get_DeployedNetworkList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedNetworkListResult {}
impl IFabricGetDeployedNetworkListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedNetworkListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedNetworkListResult_Vtbl {
        unsafe extern "system" fn get_DeployedNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedNetworkList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedNetworkList: get_DeployedNetworkList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedNetworkListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedReplicaListResult_Impl: Sized {
    fn get_DeployedReplicaList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedReplicaListResult {}
impl IFabricGetDeployedReplicaListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedReplicaListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedReplicaListResult_Vtbl {
        unsafe extern "system" fn get_DeployedReplicaList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedReplicaListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedReplicaList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedReplicaList: get_DeployedReplicaList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedReplicaListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedServicePackageListResult_Impl: Sized {
    fn get_DeployedServicePackageList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedServicePackageListResult {}
impl IFabricGetDeployedServicePackageListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedServicePackageListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedServicePackageListResult_Vtbl {
        unsafe extern "system" fn get_DeployedServicePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedServicePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedServicePackageList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedServicePackageList: get_DeployedServicePackageList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedServicePackageListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedServiceReplicaDetailResult_Impl: Sized {
    fn get_ReplicaDetail(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedServiceReplicaDetailResult {}
impl IFabricGetDeployedServiceReplicaDetailResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedServiceReplicaDetailResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedServiceReplicaDetailResult_Vtbl {
        unsafe extern "system" fn get_ReplicaDetail<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedServiceReplicaDetailResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaDetail()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaDetail: get_ReplicaDetail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedServiceReplicaDetailResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetDeployedServiceTypeListResult_Impl: Sized {
    fn get_DeployedServiceTypeList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetDeployedServiceTypeListResult {}
impl IFabricGetDeployedServiceTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedServiceTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedServiceTypeListResult_Vtbl {
        unsafe extern "system" fn get_DeployedServiceTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedServiceTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedServiceTypeList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedServiceTypeList: get_DeployedServiceTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetDeployedServiceTypeListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetNetworkApplicationListResult_Impl: Sized {
    fn get_NetworkApplicationList(
        &self,
    ) -> *mut super::super::FABRIC_NETWORK_APPLICATION_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetNetworkApplicationListResult {}
impl IFabricGetNetworkApplicationListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNetworkApplicationListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNetworkApplicationListResult_Vtbl {
        unsafe extern "system" fn get_NetworkApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NETWORK_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NetworkApplicationList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NetworkApplicationList: get_NetworkApplicationList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetNetworkApplicationListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetNetworkListResult_Impl: Sized {
    fn get_NetworkList(&self) -> *mut super::super::FABRIC_NETWORK_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetNetworkListResult {}
impl IFabricGetNetworkListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNetworkListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNetworkListResult_Vtbl {
        unsafe extern "system" fn get_NetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NETWORK_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NetworkList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NetworkList: get_NetworkList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetNetworkListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetNetworkNodeListResult_Impl: Sized {
    fn get_NetworkNodeList(&self) -> *mut super::super::FABRIC_NETWORK_NODE_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetNetworkNodeListResult {}
impl IFabricGetNetworkNodeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNetworkNodeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNetworkNodeListResult_Vtbl {
        unsafe extern "system" fn get_NetworkNodeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkNodeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NETWORK_NODE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NetworkNodeList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkNodeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NetworkNodeList: get_NetworkNodeList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetNetworkNodeListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetNodeListResult_Impl: Sized {
    fn get_NodeList(&self) -> *mut super::super::FABRIC_NODE_QUERY_RESULT_LIST;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetNodeListResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetNodeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNodeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNodeListResult_Vtbl {
        unsafe extern "system" fn get_NodeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNodeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NodeList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NodeList: get_NodeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetNodeListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetNodeListResult2_Impl: Sized + IFabricGetNodeListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetNodeListResult2 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetNodeListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNodeListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNodeListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNodeListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetNodeListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetNodeListResult2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricGetNodeListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetNodeLoadInformationResult_Impl: Sized {
    fn get_NodeLoadInformation(&self) -> *mut super::super::FABRIC_NODE_LOAD_INFORMATION;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetNodeLoadInformationResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetNodeLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNodeLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNodeLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_NodeLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNodeLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NodeLoadInformation()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NodeLoadInformation: get_NodeLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetNodeLoadInformationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetPartitionListResult_Impl: Sized {
    fn get_PartitionList(&self) -> *mut super::super::FABRIC_SERVICE_PARTITION_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetPartitionListResult {}
impl IFabricGetPartitionListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetPartitionListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetPartitionListResult_Vtbl {
        unsafe extern "system" fn get_PartitionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetPartitionListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_PARTITION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PartitionList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_PartitionList: get_PartitionList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetPartitionListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetPartitionListResult2_Impl: Sized + IFabricGetPartitionListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetPartitionListResult2 {}
impl IFabricGetPartitionListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetPartitionListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetPartitionListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetPartitionListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetPartitionListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetPartitionListResult2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricGetPartitionListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetPartitionLoadInformationResult_Impl: Sized {
    fn get_PartitionLoadInformation(&self) -> *mut super::super::FABRIC_PARTITION_LOAD_INFORMATION;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetPartitionLoadInformationResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetPartitionLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetPartitionLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetPartitionLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_PartitionLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetPartitionLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PartitionLoadInformation()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_PartitionLoadInformation: get_PartitionLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetPartitionLoadInformationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetProvisionedCodeVersionListResult_Impl: Sized {
    fn get_ProvisionedCodeVersionList(
        &self,
    ) -> *mut super::super::FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetProvisionedCodeVersionListResult {}
impl IFabricGetProvisionedCodeVersionListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetProvisionedCodeVersionListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetProvisionedCodeVersionListResult_Vtbl {
        unsafe extern "system" fn get_ProvisionedCodeVersionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetProvisionedCodeVersionListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ProvisionedCodeVersionList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ProvisionedCodeVersionList: get_ProvisionedCodeVersionList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetProvisionedCodeVersionListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetProvisionedConfigVersionListResult_Impl: Sized {
    fn get_ProvisionedConfigVersionList(
        &self,
    ) -> *mut super::super::FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetProvisionedConfigVersionListResult {}
impl IFabricGetProvisionedConfigVersionListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetProvisionedConfigVersionListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetProvisionedConfigVersionListResult_Vtbl {
        unsafe extern "system" fn get_ProvisionedConfigVersionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetProvisionedConfigVersionListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ProvisionedConfigVersionList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ProvisionedConfigVersionList: get_ProvisionedConfigVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetProvisionedConfigVersionListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetRepairTaskListResult_Impl: Sized {
    fn get_Tasks(&self) -> *mut super::super::FABRIC_REPAIR_TASK_LIST;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetRepairTaskListResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetRepairTaskListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetRepairTaskListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetRepairTaskListResult_Vtbl {
        unsafe extern "system" fn get_Tasks<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetRepairTaskListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPAIR_TASK_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Tasks()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Tasks: get_Tasks::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetRepairTaskListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetReplicaListResult_Impl: Sized {
    fn get_ReplicaList(&self) -> *mut super::super::FABRIC_SERVICE_REPLICA_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetReplicaListResult {}
impl IFabricGetReplicaListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicaListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicaListResult_Vtbl {
        unsafe extern "system" fn get_ReplicaList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetReplicaListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_REPLICA_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaList: get_ReplicaList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetReplicaListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetReplicaListResult2_Impl: Sized + IFabricGetReplicaListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetReplicaListResult2 {}
impl IFabricGetReplicaListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicaListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicaListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetReplicaListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetReplicaListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetReplicaListResult2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricGetReplicaListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricGetReplicaLoadInformationResult_Impl: Sized {
    fn get_ReplicaLoadInformation(&self) -> *mut super::super::FABRIC_REPLICA_LOAD_INFORMATION;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricGetReplicaLoadInformationResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricGetReplicaLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicaLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicaLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_ReplicaLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetReplicaLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPLICA_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaLoadInformation()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaLoadInformation: get_ReplicaLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetReplicaLoadInformationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetRollingUpgradeMonitoringPolicyResult_Impl: Sized {
    fn get_Policy(&self) -> *mut super::super::FABRIC_ROLLING_UPGRADE_MONITORING_POLICY;
}
impl ::windows_core::RuntimeName for IFabricGetRollingUpgradeMonitoringPolicyResult {}
impl IFabricGetRollingUpgradeMonitoringPolicyResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetRollingUpgradeMonitoringPolicyResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetRollingUpgradeMonitoringPolicyResult_Vtbl {
        unsafe extern "system" fn get_Policy<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetRollingUpgradeMonitoringPolicyResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ROLLING_UPGRADE_MONITORING_POLICY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Policy()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Policy: get_Policy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == & < IFabricGetRollingUpgradeMonitoringPolicyResult < > as::windows_core::ComInterface >::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetServiceGroupMemberListResult_Impl: Sized {
    fn get_ServiceGroupMemberList(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetServiceGroupMemberListResult {}
impl IFabricGetServiceGroupMemberListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceGroupMemberListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceGroupMemberListResult_Vtbl {
        unsafe extern "system" fn get_ServiceGroupMemberList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceGroupMemberListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceGroupMemberList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceGroupMemberList: get_ServiceGroupMemberList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetServiceGroupMemberListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetServiceGroupMemberTypeListResult_Impl: Sized {
    fn get_ServiceGroupMemberTypeList(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetServiceGroupMemberTypeListResult {}
impl IFabricGetServiceGroupMemberTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceGroupMemberTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceGroupMemberTypeListResult_Vtbl {
        unsafe extern "system" fn get_ServiceGroupMemberTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceGroupMemberTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceGroupMemberTypeList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceGroupMemberTypeList: get_ServiceGroupMemberTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetServiceGroupMemberTypeListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetServiceListResult_Impl: Sized {
    fn get_ServiceList(&self) -> *mut super::super::FABRIC_SERVICE_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetServiceListResult {}
impl IFabricGetServiceListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceListResult_Vtbl {
        unsafe extern "system" fn get_ServiceList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceList: get_ServiceList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetServiceListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetServiceListResult2_Impl: Sized + IFabricGetServiceListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows_core::RuntimeName for IFabricGetServiceListResult2 {}
impl IFabricGetServiceListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetServiceListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetServiceListResult2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricGetServiceListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetServiceNameResult_Impl: Sized {
    fn get_ServiceName(&self) -> *mut super::super::FABRIC_SERVICE_NAME_QUERY_RESULT;
}
impl ::windows_core::RuntimeName for IFabricGetServiceNameResult {}
impl IFabricGetServiceNameResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceNameResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceNameResult_Vtbl {
        unsafe extern "system" fn get_ServiceName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceNameResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_NAME_QUERY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceName()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceName: get_ServiceName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetServiceNameResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetServiceTypeListResult_Impl: Sized {
    fn get_ServiceTypeList(&self) -> *mut super::super::FABRIC_SERVICE_TYPE_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricGetServiceTypeListResult {}
impl IFabricGetServiceTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceTypeListResult_Vtbl {
        unsafe extern "system" fn get_ServiceTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceTypeList()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceTypeList: get_ServiceTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetServiceTypeListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricGetUnplacedReplicaInformationResult_Impl: Sized {
    fn get_UnplacedReplicaInformation(
        &self,
    ) -> *mut super::super::FABRIC_UNPLACED_REPLICA_INFORMATION;
}
impl ::windows_core::RuntimeName for IFabricGetUnplacedReplicaInformationResult {}
impl IFabricGetUnplacedReplicaInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetUnplacedReplicaInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetUnplacedReplicaInformationResult_Vtbl {
        unsafe extern "system" fn get_UnplacedReplicaInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetUnplacedReplicaInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_UNPLACED_REPLICA_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UnplacedReplicaInformation()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_UnplacedReplicaInformation: get_UnplacedReplicaInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetUnplacedReplicaInformationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricHealthClient_Impl: Sized {
    fn ReportHealth(
        &self,
        healthreport: *const super::super::FABRIC_HEALTH_REPORT,
    ) -> ::windows_core::Result<()>;
    fn BeginGetClusterHealth(
        &self,
        healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricClusterHealthResult>;
    fn BeginGetNodeHealth(
        &self,
        nodename: &::windows_core::PCWSTR,
        healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricNodeHealthResult>;
    fn BeginGetApplicationHealth(
        &self,
        applicationname: *const u16,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricApplicationHealthResult>;
    fn BeginGetServiceHealth(
        &self,
        servicename: *const u16,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricServiceHealthResult>;
    fn BeginGetPartitionHealth(
        &self,
        partitionid: &::windows_core::GUID,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPartitionHealthResult>;
    fn BeginGetReplicaHealth(
        &self,
        partitionid: &::windows_core::GUID,
        replicaid: i64,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricReplicaHealthResult>;
    fn BeginGetDeployedApplicationHealth(
        &self,
        applicationname: *const u16,
        nodename: &::windows_core::PCWSTR,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricDeployedApplicationHealthResult>;
    fn BeginGetDeployedServicePackageHealth(
        &self,
        applicationname: *const u16,
        servicemanifestname: &::windows_core::PCWSTR,
        nodename: &::windows_core::PCWSTR,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServicePackageHealth(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricDeployedServicePackageHealthResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricHealthClient {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricHealthClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient_Vtbl {
        unsafe extern "system" fn ReportHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthreport: *const super::super::FABRIC_HEALTH_REPORT,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportHealth(::core::mem::transmute_copy(&healthreport))
                .into()
        }
        unsafe extern "system" fn BeginGetClusterHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterHealth(
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterHealth(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNodeHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows_core::PCWSTR,
            healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeHealth(
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNodeHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeHealth(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationHealth(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationHealth(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceHealth(
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceHealth(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows_core::GUID,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionHealth(
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionHealth(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows_core::GUID,
            replicaid: i64,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaHealth(
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&replicaid),
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetReplicaHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaHealth(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedApplicationHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            nodename: ::windows_core::PCWSTR,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationHealth(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedApplicationHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedApplicationHealth(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServicePackageHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            servicemanifestname: ::windows_core::PCWSTR,
            nodename: ::windows_core::PCWSTR,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServicePackageHealth(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute(&servicemanifestname),
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&healthpolicy),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedServicePackageHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndGetDeployedServicePackageHealth(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportHealth: ReportHealth::<Identity, Impl, OFFSET>,
            BeginGetClusterHealth: BeginGetClusterHealth::<Identity, Impl, OFFSET>,
            EndGetClusterHealth: EndGetClusterHealth::<Identity, Impl, OFFSET>,
            BeginGetNodeHealth: BeginGetNodeHealth::<Identity, Impl, OFFSET>,
            EndGetNodeHealth: EndGetNodeHealth::<Identity, Impl, OFFSET>,
            BeginGetApplicationHealth: BeginGetApplicationHealth::<Identity, Impl, OFFSET>,
            EndGetApplicationHealth: EndGetApplicationHealth::<Identity, Impl, OFFSET>,
            BeginGetServiceHealth: BeginGetServiceHealth::<Identity, Impl, OFFSET>,
            EndGetServiceHealth: EndGetServiceHealth::<Identity, Impl, OFFSET>,
            BeginGetPartitionHealth: BeginGetPartitionHealth::<Identity, Impl, OFFSET>,
            EndGetPartitionHealth: EndGetPartitionHealth::<Identity, Impl, OFFSET>,
            BeginGetReplicaHealth: BeginGetReplicaHealth::<Identity, Impl, OFFSET>,
            EndGetReplicaHealth: EndGetReplicaHealth::<Identity, Impl, OFFSET>,
            BeginGetDeployedApplicationHealth: BeginGetDeployedApplicationHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationHealth: EndGetDeployedApplicationHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetDeployedServicePackageHealth: BeginGetDeployedServicePackageHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServicePackageHealth: EndGetDeployedServicePackageHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricHealthClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricHealthClient2_Impl: Sized + IFabricHealthClient_Impl {
    fn BeginGetClusterHealth2(
        &self,
        querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricClusterHealthResult>;
    fn BeginGetNodeHealth2(
        &self,
        querydescription: *const super::super::FABRIC_NODE_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricNodeHealthResult>;
    fn BeginGetApplicationHealth2(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricApplicationHealthResult>;
    fn BeginGetServiceHealth2(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricServiceHealthResult>;
    fn BeginGetPartitionHealth2(
        &self,
        querydescription: *const super::super::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPartitionHealthResult>;
    fn BeginGetReplicaHealth2(
        &self,
        querydescription: *const super::super::FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricReplicaHealthResult>;
    fn BeginGetDeployedApplicationHealth2(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricDeployedApplicationHealthResult>;
    fn BeginGetDeployedServicePackageHealth2(
        &self,
        querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServicePackageHealth2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricDeployedServicePackageHealthResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricHealthClient2 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricHealthClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient2_Vtbl {
        unsafe extern "system" fn BeginGetClusterHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterHealth2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNodeHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NODE_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNodeHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeHealth2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationHealth2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceHealth2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionHealth2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetReplicaHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaHealth2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedApplicationHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedApplicationHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedApplicationHealth2(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServicePackageHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServicePackageHealth2(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedServicePackageHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndGetDeployedServicePackageHealth2(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricHealthClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterHealth2: BeginGetClusterHealth2::<Identity, Impl, OFFSET>,
            EndGetClusterHealth2: EndGetClusterHealth2::<Identity, Impl, OFFSET>,
            BeginGetNodeHealth2: BeginGetNodeHealth2::<Identity, Impl, OFFSET>,
            EndGetNodeHealth2: EndGetNodeHealth2::<Identity, Impl, OFFSET>,
            BeginGetApplicationHealth2: BeginGetApplicationHealth2::<Identity, Impl, OFFSET>,
            EndGetApplicationHealth2: EndGetApplicationHealth2::<Identity, Impl, OFFSET>,
            BeginGetServiceHealth2: BeginGetServiceHealth2::<Identity, Impl, OFFSET>,
            EndGetServiceHealth2: EndGetServiceHealth2::<Identity, Impl, OFFSET>,
            BeginGetPartitionHealth2: BeginGetPartitionHealth2::<Identity, Impl, OFFSET>,
            EndGetPartitionHealth2: EndGetPartitionHealth2::<Identity, Impl, OFFSET>,
            BeginGetReplicaHealth2: BeginGetReplicaHealth2::<Identity, Impl, OFFSET>,
            EndGetReplicaHealth2: EndGetReplicaHealth2::<Identity, Impl, OFFSET>,
            BeginGetDeployedApplicationHealth2: BeginGetDeployedApplicationHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationHealth2: EndGetDeployedApplicationHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetDeployedServicePackageHealth2: BeginGetDeployedServicePackageHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServicePackageHealth2: EndGetDeployedServicePackageHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricHealthClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricHealthClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricHealthClient3_Impl: Sized + IFabricHealthClient2_Impl {
    fn BeginGetClusterHealthChunk(
        &self,
        querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterHealthChunk(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetClusterHealthChunkResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricHealthClient3 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricHealthClient3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient3_Vtbl {
        unsafe extern "system" fn BeginGetClusterHealthChunk<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterHealthChunk(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterHealthChunk<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterHealthChunk(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricHealthClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterHealthChunk: BeginGetClusterHealthChunk::<Identity, Impl, OFFSET>,
            EndGetClusterHealthChunk: EndGetClusterHealthChunk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricHealthClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricHealthClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricHealthClient2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricHealthClient4_Impl: Sized + IFabricHealthClient3_Impl {
    fn ReportHealth2(
        &self,
        healthreport: *const super::super::FABRIC_HEALTH_REPORT,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricHealthClient4 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricHealthClient4_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient4_Vtbl {
        unsafe extern "system" fn ReportHealth2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthreport: *const super::super::FABRIC_HEALTH_REPORT,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportHealth2(
                ::core::mem::transmute_copy(&healthreport),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        Self {
            base__: IFabricHealthClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportHealth2: ReportHealth2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricHealthClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricHealthClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricHealthClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricHealthClient3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricInfrastructureServiceClient_Impl: Sized {
    fn BeginInvokeInfrastructureCommand(
        &self,
        servicename: *const u16,
        command: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndInvokeInfrastructureCommand(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
    fn BeginInvokeInfrastructureQuery(
        &self,
        servicename: *const u16,
        command: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndInvokeInfrastructureQuery(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
}
impl ::windows_core::RuntimeName for IFabricInfrastructureServiceClient {}
impl IFabricInfrastructureServiceClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricInfrastructureServiceClient_Impl,
        const OFFSET: isize,
    >() -> IFabricInfrastructureServiceClient_Vtbl {
        unsafe extern "system" fn BeginInvokeInfrastructureCommand<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            command: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginInvokeInfrastructureCommand(
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&command),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInvokeInfrastructureCommand<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndInvokeInfrastructureCommand(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginInvokeInfrastructureQuery<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            command: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginInvokeInfrastructureQuery(
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&command),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInvokeInfrastructureQuery<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndInvokeInfrastructureQuery(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginInvokeInfrastructureCommand: BeginInvokeInfrastructureCommand::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndInvokeInfrastructureCommand: EndInvokeInfrastructureCommand::<Identity, Impl, OFFSET>,
            BeginInvokeInfrastructureQuery: BeginInvokeInfrastructureQuery::<Identity, Impl, OFFSET>,
            EndInvokeInfrastructureQuery: EndInvokeInfrastructureQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricInfrastructureServiceClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricMovePrimaryResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_MOVE_PRIMARY_RESULT;
}
impl ::windows_core::RuntimeName for IFabricMovePrimaryResult {}
impl IFabricMovePrimaryResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricMovePrimaryResult_Impl,
        const OFFSET: isize,
    >() -> IFabricMovePrimaryResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricMovePrimaryResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_MOVE_PRIMARY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricMovePrimaryResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricMoveSecondaryResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_MOVE_SECONDARY_RESULT;
}
impl ::windows_core::RuntimeName for IFabricMoveSecondaryResult {}
impl IFabricMoveSecondaryResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricMoveSecondaryResult_Impl,
        const OFFSET: isize,
    >() -> IFabricMoveSecondaryResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricMoveSecondaryResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_MOVE_SECONDARY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricMoveSecondaryResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricNameEnumerationResult_Impl: Sized {
    fn get_EnumerationStatus(&self) -> super::super::FABRIC_ENUMERATION_STATUS;
    fn GetNames(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut *mut u16,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricNameEnumerationResult {}
impl IFabricNameEnumerationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNameEnumerationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricNameEnumerationResult_Vtbl {
        unsafe extern "system" fn get_EnumerationStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNameEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ENUMERATION_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_EnumerationStatus()
        }
        unsafe extern "system" fn GetNames<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNameEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut *mut u16,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNames(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_EnumerationStatus: get_EnumerationStatus::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricNameEnumerationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricNetworkManagementClient_Impl: Sized {
    fn BeginCreateNetwork(
        &self,
        networkname: &::windows_core::PCWSTR,
        description: *const super::super::FABRIC_NETWORK_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateNetwork(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginDeleteNetwork(
        &self,
        deletedescription: *const super::super::FABRIC_DELETE_NETWORK_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteNetwork(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetNetworkList(
        &self,
        querydescription: *const super::super::FABRIC_NETWORK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNetworkList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetNetworkListResult>;
    fn BeginGetNetworkApplicationList(
        &self,
        querydescription: *const super::super::FABRIC_NETWORK_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNetworkApplicationList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetNetworkApplicationListResult>;
    fn BeginGetNetworkNodeList(
        &self,
        querydescription: *const super::super::FABRIC_NETWORK_NODE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNetworkNodeList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetNetworkNodeListResult>;
    fn BeginGetApplicationNetworkList(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_NETWORK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationNetworkList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetApplicationNetworkListResult>;
    fn BeginGetDeployedNetworkList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_NETWORK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedNetworkList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedNetworkListResult>;
    fn BeginGetDeployedNetworkCodePackageList(
        &self,
        querydescription : *const super::super:: FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedNetworkCodePackageList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedNetworkCodePackageListResult>;
}
impl ::windows_core::RuntimeName for IFabricNetworkManagementClient {}
impl IFabricNetworkManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNetworkManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricNetworkManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateNetwork<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            networkname: ::windows_core::PCWSTR,
            description: *const super::super::FABRIC_NETWORK_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateNetwork(
                ::core::mem::transmute(&networkname),
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateNetwork<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateNetwork(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteNetwork<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            deletedescription: *const super::super::FABRIC_DELETE_NETWORK_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteNetwork(
                ::core::mem::transmute_copy(&deletedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteNetwork<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteNetwork(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NETWORK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNetworkList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNetworkList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNetworkApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NETWORK_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNetworkApplicationList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNetworkApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNetworkApplicationList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNetworkNodeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NETWORK_NODE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNetworkNodeList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNetworkNodeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNetworkNodeList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_NETWORK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationNetworkList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationNetworkList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_NETWORK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedNetworkList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedNetworkList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedNetworkList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedNetworkCodePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedNetworkCodePackageList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedNetworkCodePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndGetDeployedNetworkCodePackageList(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateNetwork: BeginCreateNetwork::<Identity, Impl, OFFSET>,
            EndCreateNetwork: EndCreateNetwork::<Identity, Impl, OFFSET>,
            BeginDeleteNetwork: BeginDeleteNetwork::<Identity, Impl, OFFSET>,
            EndDeleteNetwork: EndDeleteNetwork::<Identity, Impl, OFFSET>,
            BeginGetNetworkList: BeginGetNetworkList::<Identity, Impl, OFFSET>,
            EndGetNetworkList: EndGetNetworkList::<Identity, Impl, OFFSET>,
            BeginGetNetworkApplicationList: BeginGetNetworkApplicationList::<Identity, Impl, OFFSET>,
            EndGetNetworkApplicationList: EndGetNetworkApplicationList::<Identity, Impl, OFFSET>,
            BeginGetNetworkNodeList: BeginGetNetworkNodeList::<Identity, Impl, OFFSET>,
            EndGetNetworkNodeList: EndGetNetworkNodeList::<Identity, Impl, OFFSET>,
            BeginGetApplicationNetworkList: BeginGetApplicationNetworkList::<Identity, Impl, OFFSET>,
            EndGetApplicationNetworkList: EndGetApplicationNetworkList::<Identity, Impl, OFFSET>,
            BeginGetDeployedNetworkList: BeginGetDeployedNetworkList::<Identity, Impl, OFFSET>,
            EndGetDeployedNetworkList: EndGetDeployedNetworkList::<Identity, Impl, OFFSET>,
            BeginGetDeployedNetworkCodePackageList: BeginGetDeployedNetworkCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedNetworkCodePackageList: EndGetDeployedNetworkCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricNetworkManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricNodeHealthResult_Impl: Sized {
    fn get_NodeHealth(&self) -> *mut super::super::FABRIC_NODE_HEALTH;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricNodeHealthResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricNodeHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNodeHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricNodeHealthResult_Vtbl {
        unsafe extern "system" fn get_NodeHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNodeHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NodeHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NodeHealth: get_NodeHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricNodeHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricNodeTransitionProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_NODE_TRANSITION_PROGRESS;
}
impl ::windows_core::RuntimeName for IFabricNodeTransitionProgressResult {}
impl IFabricNodeTransitionProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNodeTransitionProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricNodeTransitionProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNodeTransitionProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_TRANSITION_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricNodeTransitionProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricOrchestrationUpgradeStatusResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_ORCHESTRATION_UPGRADE_PROGRESS;
}
impl ::windows_core::RuntimeName for IFabricOrchestrationUpgradeStatusResult {}
impl IFabricOrchestrationUpgradeStatusResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricOrchestrationUpgradeStatusResult_Impl,
        const OFFSET: isize,
    >() -> IFabricOrchestrationUpgradeStatusResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOrchestrationUpgradeStatusResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ORCHESTRATION_UPGRADE_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricOrchestrationUpgradeStatusResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricPartitionDataLossProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_PARTITION_DATA_LOSS_PROGRESS;
}
impl ::windows_core::RuntimeName for IFabricPartitionDataLossProgressResult {}
impl IFabricPartitionDataLossProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionDataLossProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionDataLossProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionDataLossProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_DATA_LOSS_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPartitionDataLossProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricPartitionHealthResult_Impl: Sized {
    fn get_PartitionHealth(&self) -> *mut super::super::FABRIC_PARTITION_HEALTH;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricPartitionHealthResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricPartitionHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionHealthResult_Vtbl {
        unsafe extern "system" fn get_PartitionHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PartitionHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_PartitionHealth: get_PartitionHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPartitionHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricPartitionQuorumLossProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_PARTITION_QUORUM_LOSS_PROGRESS;
}
impl ::windows_core::RuntimeName for IFabricPartitionQuorumLossProgressResult {}
impl IFabricPartitionQuorumLossProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionQuorumLossProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionQuorumLossProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionQuorumLossProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_QUORUM_LOSS_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPartitionQuorumLossProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricPartitionRestartProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_PARTITION_RESTART_PROGRESS;
}
impl ::windows_core::RuntimeName for IFabricPartitionRestartProgressResult {}
impl IFabricPartitionRestartProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionRestartProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionRestartProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionRestartProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_RESTART_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPartitionRestartProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricPropertyBatchResult_Impl: Sized {
    fn GetProperty(
        &self,
        operationindexinrequest: u32,
    ) -> ::windows_core::Result<IFabricPropertyValueResult>;
}
impl ::windows_core::RuntimeName for IFabricPropertyBatchResult {}
impl IFabricPropertyBatchResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyBatchResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyBatchResult_Vtbl {
        unsafe extern "system" fn GetProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyBatchResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationindexinrequest: u32,
            property: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&operationindexinrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPropertyBatchResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricPropertyEnumerationResult_Impl: Sized {
    fn get_EnumerationStatus(&self) -> super::super::FABRIC_ENUMERATION_STATUS;
    fn get_PropertyCount(&self) -> u32;
    fn GetProperty(&self, index: u32) -> ::windows_core::Result<IFabricPropertyValueResult>;
}
impl ::windows_core::RuntimeName for IFabricPropertyEnumerationResult {}
impl IFabricPropertyEnumerationResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyEnumerationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyEnumerationResult_Vtbl {
        unsafe extern "system" fn get_EnumerationStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ENUMERATION_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_EnumerationStatus()
        }
        unsafe extern "system" fn get_PropertyCount<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PropertyCount()
        }
        unsafe extern "system" fn GetProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
            property: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_EnumerationStatus: get_EnumerationStatus::<Identity, Impl, OFFSET>,
            get_PropertyCount: get_PropertyCount::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPropertyEnumerationResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricPropertyManagementClient_Impl: Sized {
    fn BeginCreateName(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateName(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginDeleteName(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteName(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginNameExists(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndNameExists(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<u8>;
    fn BeginEnumerateSubNames(
        &self,
        name: *const u16,
        previousresult: ::core::option::Option<&IFabricNameEnumerationResult>,
        recursive: ::windows::Win32::Foundation::BOOLEAN,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndEnumerateSubNames(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricNameEnumerationResult>;
    fn BeginPutPropertyBinary(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        datalength: u32,
        data: *const u8,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyBinary(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginPutPropertyInt64(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        data: i64,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyInt64(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginPutPropertyDouble(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        data: f64,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyDouble(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginPutPropertyWString(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        data: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyWString(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginPutPropertyGuid(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        data: *const ::windows_core::GUID,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyGuid(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginDeleteProperty(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteProperty(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetPropertyMetadata(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPropertyMetadata(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPropertyMetadataResult>;
    fn BeginGetProperty(
        &self,
        name: *const u16,
        propertyname: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetProperty(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPropertyValueResult>;
    fn BeginSubmitPropertyBatch(
        &self,
        name: *const u16,
        operationcount: u32,
        operations: *const super::super::FABRIC_PROPERTY_BATCH_OPERATION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndSubmitPropertyBatch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
        failedoperationindexinrequest: *mut u32,
        result: *mut ::core::option::Option<IFabricPropertyBatchResult>,
    ) -> ::windows_core::Result<()>;
    fn BeginEnumerateProperties(
        &self,
        name: *const u16,
        includevalues: ::windows::Win32::Foundation::BOOLEAN,
        previousresult: ::core::option::Option<&IFabricPropertyEnumerationResult>,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndEnumerateProperties(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPropertyEnumerationResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricPropertyManagementClient {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricPropertyManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateName(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateName(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteName(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteName(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginNameExists<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginNameExists(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndNameExists<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            value: *mut u8,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndNameExists(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumerateSubNames<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            previousresult: *mut ::core::ffi::c_void,
            recursive: ::windows::Win32::Foundation::BOOLEAN,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginEnumerateSubNames(
                ::core::mem::transmute_copy(&name),
                ::windows_core::from_raw_borrowed(&previousresult),
                ::core::mem::transmute_copy(&recursive),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEnumerateSubNames<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndEnumerateSubNames(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPutPropertyBinary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            datalength: u32,
            data: *const u8,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyBinary(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&datalength),
                ::core::mem::transmute_copy(&data),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPutPropertyBinary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyBinary(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyInt64<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            data: i64,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyInt64(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&data),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPutPropertyInt64<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyInt64(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyDouble<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            data: f64,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyDouble(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&data),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPutPropertyDouble<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyDouble(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyWString<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            data: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyWString(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute(&data),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPutPropertyWString<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyWString(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyGuid<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            data: *const ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyGuid(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&data),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPutPropertyGuid<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyGuid(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteProperty(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteProperty(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPropertyMetadata<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPropertyMetadata(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPropertyMetadata<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPropertyMetadata(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetProperty(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetProperty(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSubmitPropertyBatch<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            operationcount: u32,
            operations: *const super::super::FABRIC_PROPERTY_BATCH_OPERATION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSubmitPropertyBatch(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&operationcount),
                ::core::mem::transmute_copy(&operations),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubmitPropertyBatch<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            failedoperationindexinrequest: *mut u32,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSubmitPropertyBatch(
                ::windows_core::from_raw_borrowed(&context),
                ::core::mem::transmute_copy(&failedoperationindexinrequest),
                ::core::mem::transmute_copy(&result),
            )
            .into()
        }
        unsafe extern "system" fn BeginEnumerateProperties<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            includevalues: ::windows::Win32::Foundation::BOOLEAN,
            previousresult: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginEnumerateProperties(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&includevalues),
                ::windows_core::from_raw_borrowed(&previousresult),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEnumerateProperties<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndEnumerateProperties(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateName: BeginCreateName::<Identity, Impl, OFFSET>,
            EndCreateName: EndCreateName::<Identity, Impl, OFFSET>,
            BeginDeleteName: BeginDeleteName::<Identity, Impl, OFFSET>,
            EndDeleteName: EndDeleteName::<Identity, Impl, OFFSET>,
            BeginNameExists: BeginNameExists::<Identity, Impl, OFFSET>,
            EndNameExists: EndNameExists::<Identity, Impl, OFFSET>,
            BeginEnumerateSubNames: BeginEnumerateSubNames::<Identity, Impl, OFFSET>,
            EndEnumerateSubNames: EndEnumerateSubNames::<Identity, Impl, OFFSET>,
            BeginPutPropertyBinary: BeginPutPropertyBinary::<Identity, Impl, OFFSET>,
            EndPutPropertyBinary: EndPutPropertyBinary::<Identity, Impl, OFFSET>,
            BeginPutPropertyInt64: BeginPutPropertyInt64::<Identity, Impl, OFFSET>,
            EndPutPropertyInt64: EndPutPropertyInt64::<Identity, Impl, OFFSET>,
            BeginPutPropertyDouble: BeginPutPropertyDouble::<Identity, Impl, OFFSET>,
            EndPutPropertyDouble: EndPutPropertyDouble::<Identity, Impl, OFFSET>,
            BeginPutPropertyWString: BeginPutPropertyWString::<Identity, Impl, OFFSET>,
            EndPutPropertyWString: EndPutPropertyWString::<Identity, Impl, OFFSET>,
            BeginPutPropertyGuid: BeginPutPropertyGuid::<Identity, Impl, OFFSET>,
            EndPutPropertyGuid: EndPutPropertyGuid::<Identity, Impl, OFFSET>,
            BeginDeleteProperty: BeginDeleteProperty::<Identity, Impl, OFFSET>,
            EndDeleteProperty: EndDeleteProperty::<Identity, Impl, OFFSET>,
            BeginGetPropertyMetadata: BeginGetPropertyMetadata::<Identity, Impl, OFFSET>,
            EndGetPropertyMetadata: EndGetPropertyMetadata::<Identity, Impl, OFFSET>,
            BeginGetProperty: BeginGetProperty::<Identity, Impl, OFFSET>,
            EndGetProperty: EndGetProperty::<Identity, Impl, OFFSET>,
            BeginSubmitPropertyBatch: BeginSubmitPropertyBatch::<Identity, Impl, OFFSET>,
            EndSubmitPropertyBatch: EndSubmitPropertyBatch::<Identity, Impl, OFFSET>,
            BeginEnumerateProperties: BeginEnumerateProperties::<Identity, Impl, OFFSET>,
            EndEnumerateProperties: EndEnumerateProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPropertyManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricPropertyManagementClient2_Impl:
    Sized + IFabricPropertyManagementClient_Impl
{
    fn BeginPutCustomPropertyOperation(
        &self,
        name: *const u16,
        propertyoperation: *const super::super::FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutCustomPropertyOperation(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricPropertyManagementClient2 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricPropertyManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyManagementClient2_Vtbl {
        unsafe extern "system" fn BeginPutCustomPropertyOperation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyoperation: *const super::super::FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutCustomPropertyOperation(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&propertyoperation),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPutCustomPropertyOperation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutCustomPropertyOperation(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricPropertyManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginPutCustomPropertyOperation: BeginPutCustomPropertyOperation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndPutCustomPropertyOperation: EndPutCustomPropertyOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPropertyManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricPropertyManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricPropertyMetadataResult_Impl: Sized {
    fn get_Metadata(&self) -> *mut super::super::FABRIC_NAMED_PROPERTY_METADATA;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricPropertyMetadataResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricPropertyMetadataResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyMetadataResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyMetadataResult_Vtbl {
        unsafe extern "system" fn get_Metadata<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyMetadataResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NAMED_PROPERTY_METADATA {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Metadata()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Metadata: get_Metadata::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPropertyMetadataResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricPropertyValueResult_Impl: Sized {
    fn get_Property(&self) -> *mut super::super::FABRIC_NAMED_PROPERTY;
    fn GetValueAsBinary(
        &self,
        bytecount: *mut u32,
        bufferedvalue: *mut *mut u8,
    ) -> ::windows_core::Result<()>;
    fn GetValueAsInt64(&self) -> ::windows_core::Result<i64>;
    fn GetValueAsDouble(&self) -> ::windows_core::Result<f64>;
    fn GetValueAsWString(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetValueAsGuid(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricPropertyValueResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricPropertyValueResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyValueResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyValueResult_Vtbl {
        unsafe extern "system" fn get_Property<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NAMED_PROPERTY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Property()
        }
        unsafe extern "system" fn GetValueAsBinary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bytecount: *mut u32,
            bufferedvalue: *mut *mut u8,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValueAsBinary(
                ::core::mem::transmute_copy(&bytecount),
                ::core::mem::transmute_copy(&bufferedvalue),
            )
            .into()
        }
        unsafe extern "system" fn GetValueAsInt64<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsInt64() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsDouble<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut f64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsDouble() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsWString<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bufferedvalue: *mut ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsWString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsGuid<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::windows_core::GUID,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Property: get_Property::<Identity, Impl, OFFSET>,
            GetValueAsBinary: GetValueAsBinary::<Identity, Impl, OFFSET>,
            GetValueAsInt64: GetValueAsInt64::<Identity, Impl, OFFSET>,
            GetValueAsDouble: GetValueAsDouble::<Identity, Impl, OFFSET>,
            GetValueAsWString: GetValueAsWString::<Identity, Impl, OFFSET>,
            GetValueAsGuid: GetValueAsGuid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricPropertyValueResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricQueryClient_Impl: Sized {
    fn BeginGetNodeList(
        &self,
        querydescription: *const super::super::FABRIC_NODE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetNodeListResult>;
    fn BeginGetApplicationTypeList(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationTypeList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetApplicationTypeListResult>;
    fn BeginGetServiceTypeList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceTypeList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetServiceTypeListResult>;
    fn BeginGetApplicationList(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetApplicationListResult>;
    fn BeginGetServiceList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetServiceListResult>;
    fn BeginGetPartitionList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetPartitionListResult>;
    fn BeginGetReplicaList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetReplicaListResult>;
    fn BeginGetDeployedApplicationList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedApplicationListResult>;
    fn BeginGetDeployedServicePackageList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServicePackageList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedServicePackageListResult>;
    fn BeginGetDeployedServiceTypeList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServiceTypeList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedServiceTypeListResult>;
    fn BeginGetDeployedCodePackageList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedCodePackageList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedCodePackageListResult>;
    fn BeginGetDeployedReplicaList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedReplicaList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedReplicaListResult>;
}
impl ::windows_core::RuntimeName for IFabricQueryClient {}
impl IFabricQueryClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient_Vtbl {
        unsafe extern "system" fn BeginGetNodeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NODE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNodeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationTypeList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationTypeList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceTypeList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceTypeList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetReplicaList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedApplicationList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedApplicationList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServicePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServicePackageList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedServicePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedServicePackageList(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServiceTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServiceTypeList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedServiceTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedServiceTypeList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedCodePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedCodePackageList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedCodePackageList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedCodePackageList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedReplicaList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedReplicaList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedReplicaList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedReplicaList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetNodeList: BeginGetNodeList::<Identity, Impl, OFFSET>,
            EndGetNodeList: EndGetNodeList::<Identity, Impl, OFFSET>,
            BeginGetApplicationTypeList: BeginGetApplicationTypeList::<Identity, Impl, OFFSET>,
            EndGetApplicationTypeList: EndGetApplicationTypeList::<Identity, Impl, OFFSET>,
            BeginGetServiceTypeList: BeginGetServiceTypeList::<Identity, Impl, OFFSET>,
            EndGetServiceTypeList: EndGetServiceTypeList::<Identity, Impl, OFFSET>,
            BeginGetApplicationList: BeginGetApplicationList::<Identity, Impl, OFFSET>,
            EndGetApplicationList: EndGetApplicationList::<Identity, Impl, OFFSET>,
            BeginGetServiceList: BeginGetServiceList::<Identity, Impl, OFFSET>,
            EndGetServiceList: EndGetServiceList::<Identity, Impl, OFFSET>,
            BeginGetPartitionList: BeginGetPartitionList::<Identity, Impl, OFFSET>,
            EndGetPartitionList: EndGetPartitionList::<Identity, Impl, OFFSET>,
            BeginGetReplicaList: BeginGetReplicaList::<Identity, Impl, OFFSET>,
            EndGetReplicaList: EndGetReplicaList::<Identity, Impl, OFFSET>,
            BeginGetDeployedApplicationList: BeginGetDeployedApplicationList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationList: EndGetDeployedApplicationList::<Identity, Impl, OFFSET>,
            BeginGetDeployedServicePackageList: BeginGetDeployedServicePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServicePackageList: EndGetDeployedServicePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetDeployedServiceTypeList: BeginGetDeployedServiceTypeList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServiceTypeList: EndGetDeployedServiceTypeList::<Identity, Impl, OFFSET>,
            BeginGetDeployedCodePackageList: BeginGetDeployedCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedCodePackageList: EndGetDeployedCodePackageList::<Identity, Impl, OFFSET>,
            BeginGetDeployedReplicaList: BeginGetDeployedReplicaList::<Identity, Impl, OFFSET>,
            EndGetDeployedReplicaList: EndGetDeployedReplicaList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricQueryClient10_Impl: Sized + IFabricQueryClient9_Impl {
    fn BeginGetDeployedApplicationPagedList(
        &self,
        querydescription: *const super::super::FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationPagedList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedApplicationPagedListResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricQueryClient10 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricQueryClient10_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient10_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient10_Vtbl {
        unsafe extern "system" fn BeginGetDeployedApplicationPagedList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationPagedList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedApplicationPagedList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndGetDeployedApplicationPagedList(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient9_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetDeployedApplicationPagedList: BeginGetDeployedApplicationPagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationPagedList: EndGetDeployedApplicationPagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient10 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient8 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient9 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricQueryClient2_Impl: Sized + IFabricQueryClient_Impl {
    fn BeginGetDeployedReplicaDetail(
        &self,
        querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedReplicaDetail(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetDeployedServiceReplicaDetailResult>;
    fn BeginGetClusterLoadInformation(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterLoadInformation(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetClusterLoadInformationResult>;
    fn BeginGetPartitionLoadInformation(
        &self,
        querydescription: *const super::super::FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionLoadInformation(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetPartitionLoadInformationResult>;
    fn BeginGetProvisionedFabricCodeVersionList(
        &self,
        querydescription: *const super::super::FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetProvisionedFabricCodeVersionList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetProvisionedCodeVersionListResult>;
    fn BeginGetProvisionedFabricConfigVersionList(
        &self,
        querydescription: *const super::super::FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetProvisionedFabricConfigVersionList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetProvisionedConfigVersionListResult>;
}
impl ::windows_core::RuntimeName for IFabricQueryClient2 {}
impl IFabricQueryClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient2_Vtbl {
        unsafe extern "system" fn BeginGetDeployedReplicaDetail<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedReplicaDetail(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetDeployedReplicaDetail<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedReplicaDetail(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetClusterLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterLoadInformation(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetClusterLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterLoadInformation(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionLoadInformation(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetProvisionedFabricCodeVersionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetProvisionedFabricCodeVersionList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetProvisionedFabricCodeVersionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndGetProvisionedFabricCodeVersionList(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetProvisionedFabricConfigVersionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetProvisionedFabricConfigVersionList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetProvisionedFabricConfigVersionList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetProvisionedFabricConfigVersionList(::windows_core::from_raw_borrowed(
                &context,
            )) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetDeployedReplicaDetail: BeginGetDeployedReplicaDetail::<Identity, Impl, OFFSET>,
            EndGetDeployedReplicaDetail: EndGetDeployedReplicaDetail::<Identity, Impl, OFFSET>,
            BeginGetClusterLoadInformation: BeginGetClusterLoadInformation::<Identity, Impl, OFFSET>,
            EndGetClusterLoadInformation: EndGetClusterLoadInformation::<Identity, Impl, OFFSET>,
            BeginGetPartitionLoadInformation: BeginGetPartitionLoadInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionLoadInformation: EndGetPartitionLoadInformation::<Identity, Impl, OFFSET>,
            BeginGetProvisionedFabricCodeVersionList: BeginGetProvisionedFabricCodeVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetProvisionedFabricCodeVersionList: EndGetProvisionedFabricCodeVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetProvisionedFabricConfigVersionList: BeginGetProvisionedFabricConfigVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetProvisionedFabricConfigVersionList: EndGetProvisionedFabricConfigVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricQueryClient3_Impl: Sized + IFabricQueryClient2_Impl {
    fn BeginGetNodeLoadInformation(
        &self,
        querydescription: *const super::super::FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeLoadInformation(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetNodeLoadInformationResult>;
    fn BeginGetReplicaLoadInformation(
        &self,
        querydescription: *const super::super::FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaLoadInformation(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetReplicaLoadInformationResult>;
}
impl ::windows_core::RuntimeName for IFabricQueryClient3 {}
impl IFabricQueryClient3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient3_Vtbl {
        unsafe extern "system" fn BeginGetNodeLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNodeLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeLoadInformation(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetReplicaLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaLoadInformation(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetNodeLoadInformation: BeginGetNodeLoadInformation::<Identity, Impl, OFFSET>,
            EndGetNodeLoadInformation: EndGetNodeLoadInformation::<Identity, Impl, OFFSET>,
            BeginGetReplicaLoadInformation: BeginGetReplicaLoadInformation::<Identity, Impl, OFFSET>,
            EndGetReplicaLoadInformation: EndGetReplicaLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricQueryClient4_Impl: Sized + IFabricQueryClient3_Impl {
    fn BeginGetServiceGroupMemberList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceGroupMemberList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetServiceGroupMemberListResult>;
    fn BeginGetServiceGroupMemberTypeList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceGroupMemberTypeList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetServiceGroupMemberTypeListResult>;
}
impl ::windows_core::RuntimeName for IFabricQueryClient4 {}
impl IFabricQueryClient4_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient4_Vtbl {
        unsafe extern "system" fn BeginGetServiceGroupMemberList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceGroupMemberList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceGroupMemberList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceGroupMemberList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceGroupMemberTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceGroupMemberTypeList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceGroupMemberTypeList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceGroupMemberTypeList(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetServiceGroupMemberList: BeginGetServiceGroupMemberList::<Identity, Impl, OFFSET>,
            EndGetServiceGroupMemberList: EndGetServiceGroupMemberList::<Identity, Impl, OFFSET>,
            BeginGetServiceGroupMemberTypeList: BeginGetServiceGroupMemberTypeList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetServiceGroupMemberTypeList: EndGetServiceGroupMemberTypeList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricQueryClient5_Impl: Sized + IFabricQueryClient4_Impl {
    fn BeginGetUnplacedReplicaInformation(
        &self,
        querydescription : *const super::super:: FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetUnplacedReplicaInformation(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetUnplacedReplicaInformationResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricQueryClient5 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricQueryClient5_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient5_Vtbl {
        unsafe extern "system" fn BeginGetUnplacedReplicaInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetUnplacedReplicaInformation(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetUnplacedReplicaInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetUnplacedReplicaInformation(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetUnplacedReplicaInformation: BeginGetUnplacedReplicaInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetUnplacedReplicaInformation: EndGetUnplacedReplicaInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient4 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricQueryClient6_Impl: Sized + IFabricQueryClient5_Impl {
    fn EndGetNodeList2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetNodeListResult2>;
    fn EndGetApplicationList2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetApplicationListResult2>;
    fn EndGetServiceList2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetServiceListResult2>;
    fn EndGetPartitionList2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetPartitionListResult2>;
    fn EndGetReplicaList2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetReplicaListResult2>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricQueryClient6 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricQueryClient6_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient6_Vtbl {
        unsafe extern "system" fn EndGetNodeList2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeList2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationList2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationList2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceList2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceList2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionList2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionList2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetReplicaList2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaList2(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            EndGetNodeList2: EndGetNodeList2::<Identity, Impl, OFFSET>,
            EndGetApplicationList2: EndGetApplicationList2::<Identity, Impl, OFFSET>,
            EndGetServiceList2: EndGetServiceList2::<Identity, Impl, OFFSET>,
            EndGetPartitionList2: EndGetPartitionList2::<Identity, Impl, OFFSET>,
            EndGetReplicaList2: EndGetReplicaList2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient5 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricQueryClient7_Impl: Sized + IFabricQueryClient6_Impl {
    fn BeginGetApplicationLoadInformation(
        &self,
        querydescription : *const super::super:: FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationLoadInformation(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetApplicationLoadInformationResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricQueryClient7 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricQueryClient7_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient7_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient7_Vtbl {
        unsafe extern "system" fn BeginGetApplicationLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationLoadInformation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationLoadInformation(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient6_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetApplicationLoadInformation: BeginGetApplicationLoadInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetApplicationLoadInformation: EndGetApplicationLoadInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient6 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricQueryClient8_Impl: Sized + IFabricQueryClient7_Impl {
    fn BeginGetServiceName(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_NAME_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceName(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetServiceNameResult>;
    fn BeginGetApplicationName(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationName(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetApplicationNameResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricQueryClient8 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricQueryClient8_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient8_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient8_Vtbl {
        unsafe extern "system" fn BeginGetServiceName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_NAME_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceName(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceName(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationName(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationName(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient7_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetServiceName: BeginGetServiceName::<Identity, Impl, OFFSET>,
            EndGetServiceName: EndGetServiceName::<Identity, Impl, OFFSET>,
            BeginGetApplicationName: BeginGetApplicationName::<Identity, Impl, OFFSET>,
            EndGetApplicationName: EndGetApplicationName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient8 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient7 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricQueryClient9_Impl: Sized + IFabricQueryClient8_Impl {
    fn BeginGetApplicationTypePagedList(
        &self,
        querydescription: *const super::super::PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationTypePagedList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetApplicationTypePagedListResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricQueryClient9 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricQueryClient9_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient9_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient9_Vtbl {
        unsafe extern "system" fn BeginGetApplicationTypePagedList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationTypePagedList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationTypePagedList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationTypePagedList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient8_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetApplicationTypePagedList: BeginGetApplicationTypePagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetApplicationTypePagedList: EndGetApplicationTypePagedList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricQueryClient9 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient7 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricQueryClient8 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricRepairManagementClient_Impl: Sized {
    fn BeginCreateRepairTask(
        &self,
        repairtask: *const super::super::FABRIC_REPAIR_TASK,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateRepairTask(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<i64>;
    fn BeginCancelRepairTask(
        &self,
        requestdescription: *const super::super::FABRIC_REPAIR_CANCEL_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCancelRepairTask(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<i64>;
    fn BeginForceApproveRepairTask(
        &self,
        requestdescription: *const super::super::FABRIC_REPAIR_APPROVE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndForceApproveRepairTask(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<i64>;
    fn BeginDeleteRepairTask(
        &self,
        requestdescription: *const super::super::FABRIC_REPAIR_DELETE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteRepairTask(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginUpdateRepairExecutionState(
        &self,
        repairtask: *const super::super::FABRIC_REPAIR_TASK,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateRepairExecutionState(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<i64>;
    fn BeginGetRepairTaskList(
        &self,
        querydescription: *const super::super::FABRIC_REPAIR_TASK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetRepairTaskList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricGetRepairTaskListResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricRepairManagementClient {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricRepairManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRepairManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricRepairManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            repairtask: *const super::super::FABRIC_REPAIR_TASK,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateRepairTask(
                ::core::mem::transmute_copy(&repairtask),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndCreateRepairTask(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCancelRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            requestdescription: *const super::super::FABRIC_REPAIR_CANCEL_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCancelRepairTask(
                ::core::mem::transmute_copy(&requestdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCancelRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndCancelRepairTask(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginForceApproveRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            requestdescription: *const super::super::FABRIC_REPAIR_APPROVE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginForceApproveRepairTask(
                ::core::mem::transmute_copy(&requestdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndForceApproveRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndForceApproveRepairTask(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDeleteRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            requestdescription: *const super::super::FABRIC_REPAIR_DELETE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteRepairTask(
                ::core::mem::transmute_copy(&requestdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteRepairTask<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteRepairTask(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginUpdateRepairExecutionState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            repairtask: *const super::super::FABRIC_REPAIR_TASK,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateRepairExecutionState(
                ::core::mem::transmute_copy(&repairtask),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateRepairExecutionState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndUpdateRepairExecutionState(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetRepairTaskList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_REPAIR_TASK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetRepairTaskList(
                ::core::mem::transmute_copy(&querydescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetRepairTaskList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetRepairTaskList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateRepairTask: BeginCreateRepairTask::<Identity, Impl, OFFSET>,
            EndCreateRepairTask: EndCreateRepairTask::<Identity, Impl, OFFSET>,
            BeginCancelRepairTask: BeginCancelRepairTask::<Identity, Impl, OFFSET>,
            EndCancelRepairTask: EndCancelRepairTask::<Identity, Impl, OFFSET>,
            BeginForceApproveRepairTask: BeginForceApproveRepairTask::<Identity, Impl, OFFSET>,
            EndForceApproveRepairTask: EndForceApproveRepairTask::<Identity, Impl, OFFSET>,
            BeginDeleteRepairTask: BeginDeleteRepairTask::<Identity, Impl, OFFSET>,
            EndDeleteRepairTask: EndDeleteRepairTask::<Identity, Impl, OFFSET>,
            BeginUpdateRepairExecutionState: BeginUpdateRepairExecutionState::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUpdateRepairExecutionState: EndUpdateRepairExecutionState::<Identity, Impl, OFFSET>,
            BeginGetRepairTaskList: BeginGetRepairTaskList::<Identity, Impl, OFFSET>,
            EndGetRepairTaskList: EndGetRepairTaskList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricRepairManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricRepairManagementClient2_Impl: Sized + IFabricRepairManagementClient_Impl {
    fn BeginUpdateRepairTaskHealthPolicy(
        &self,
        updatedescription: *const super::super::FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateRepairTaskHealthPolicy(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricRepairManagementClient2 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricRepairManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRepairManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricRepairManagementClient2_Vtbl {
        unsafe extern "system" fn BeginUpdateRepairTaskHealthPolicy<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updatedescription : *const super::super:: FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateRepairTaskHealthPolicy(
                ::core::mem::transmute_copy(&updatedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateRepairTaskHealthPolicy<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndUpdateRepairTaskHealthPolicy(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricRepairManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateRepairTaskHealthPolicy: BeginUpdateRepairTaskHealthPolicy::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUpdateRepairTaskHealthPolicy: EndUpdateRepairTaskHealthPolicy::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricRepairManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricRepairManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricReplicaHealthResult_Impl: Sized {
    fn get_ReplicaHealth(&self) -> *mut super::super::FABRIC_REPLICA_HEALTH;
}
impl ::windows_core::RuntimeName for IFabricReplicaHealthResult {}
impl IFabricReplicaHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricReplicaHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricReplicaHealthResult_Vtbl {
        unsafe extern "system" fn get_ReplicaHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicaHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPLICA_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaHealth: get_ReplicaHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricReplicaHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricResolvedServicePartitionResult_Impl: Sized {
    fn get_Partition(&self) -> *mut super::super::FABRIC_RESOLVED_SERVICE_PARTITION;
    fn GetEndpoint(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_RESOLVED_SERVICE_ENDPOINT>;
    fn CompareVersion(
        &self,
        other: ::core::option::Option<&IFabricResolvedServicePartitionResult>,
    ) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IFabricResolvedServicePartitionResult {}
impl IFabricResolvedServicePartitionResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricResolvedServicePartitionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricResolvedServicePartitionResult_Vtbl {
        unsafe extern "system" fn get_Partition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricResolvedServicePartitionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RESOLVED_SERVICE_PARTITION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Partition()
        }
        unsafe extern "system" fn GetEndpoint<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricResolvedServicePartitionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            endpoint: *mut *mut super::super::FABRIC_RESOLVED_SERVICE_ENDPOINT,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndpoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(endpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareVersion<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricResolvedServicePartitionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            other: *mut ::core::ffi::c_void,
            compareresult: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareVersion(::windows_core::from_raw_borrowed(&other)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compareresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Partition: get_Partition::<Identity, Impl, OFFSET>,
            GetEndpoint: GetEndpoint::<Identity, Impl, OFFSET>,
            CompareVersion: CompareVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricResolvedServicePartitionResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricRestartDeployedCodePackageResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_RESULT;
}
impl ::windows_core::RuntimeName for IFabricRestartDeployedCodePackageResult {}
impl IFabricRestartDeployedCodePackageResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRestartDeployedCodePackageResult_Impl,
        const OFFSET: isize,
    >() -> IFabricRestartDeployedCodePackageResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRestartDeployedCodePackageResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricRestartDeployedCodePackageResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricRestartNodeResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_NODE_RESULT;
}
impl ::windows_core::RuntimeName for IFabricRestartNodeResult {}
impl IFabricRestartNodeResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRestartNodeResult_Impl,
        const OFFSET: isize,
    >() -> IFabricRestartNodeResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRestartNodeResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricRestartNodeResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricSecretReferencesResult_Impl: Sized {
    fn get_SecretReferences(&self) -> *mut super::super::FABRIC_SECRET_REFERENCE_LIST;
}
impl ::windows_core::RuntimeName for IFabricSecretReferencesResult {}
impl IFabricSecretReferencesResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecretReferencesResult_Impl,
        const OFFSET: isize,
    >() -> IFabricSecretReferencesResult_Vtbl {
        unsafe extern "system" fn get_SecretReferences<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretReferencesResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SECRET_REFERENCE_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_SecretReferences()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_SecretReferences: get_SecretReferences::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricSecretReferencesResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricSecretStoreClient_Impl: Sized {
    fn BeginGetSecrets(
        &self,
        secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
        includevalue: ::windows::Win32::Foundation::BOOLEAN,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetSecrets(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricSecretsResult>;
    fn BeginSetSecrets(
        &self,
        secrets: *const super::super::FABRIC_SECRET_LIST,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndSetSecrets(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricSecretsResult>;
    fn BeginRemoveSecrets(
        &self,
        secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRemoveSecrets(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricSecretReferencesResult>;
    fn BeginGetSecretVersions(
        &self,
        secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetSecretVersions(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricSecretReferencesResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricSecretStoreClient {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricSecretStoreClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecretStoreClient_Impl,
        const OFFSET: isize,
    >() -> IFabricSecretStoreClient_Vtbl {
        unsafe extern "system" fn BeginGetSecrets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
            includevalue: ::windows::Win32::Foundation::BOOLEAN,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetSecrets(
                ::core::mem::transmute_copy(&secretreferences),
                ::core::mem::transmute_copy(&includevalue),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetSecrets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetSecrets(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSetSecrets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secrets: *const super::super::FABRIC_SECRET_LIST,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSetSecrets(
                ::core::mem::transmute_copy(&secrets),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSetSecrets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndSetSecrets(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRemoveSecrets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRemoveSecrets(
                ::core::mem::transmute_copy(&secretreferences),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRemoveSecrets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRemoveSecrets(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetSecretVersions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetSecretVersions(
                ::core::mem::transmute_copy(&secretreferences),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetSecretVersions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetSecretVersions(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetSecrets: BeginGetSecrets::<Identity, Impl, OFFSET>,
            EndGetSecrets: EndGetSecrets::<Identity, Impl, OFFSET>,
            BeginSetSecrets: BeginSetSecrets::<Identity, Impl, OFFSET>,
            EndSetSecrets: EndSetSecrets::<Identity, Impl, OFFSET>,
            BeginRemoveSecrets: BeginRemoveSecrets::<Identity, Impl, OFFSET>,
            EndRemoveSecrets: EndRemoveSecrets::<Identity, Impl, OFFSET>,
            BeginGetSecretVersions: BeginGetSecretVersions::<Identity, Impl, OFFSET>,
            EndGetSecretVersions: EndGetSecretVersions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricSecretStoreClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricSecretsResult_Impl: Sized {
    fn get_Secrets(&self) -> *mut super::super::FABRIC_SECRET_LIST;
}
impl ::windows_core::RuntimeName for IFabricSecretsResult {}
impl IFabricSecretsResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecretsResult_Impl,
        const OFFSET: isize,
    >() -> IFabricSecretsResult_Vtbl {
        unsafe extern "system" fn get_Secrets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretsResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SECRET_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Secrets()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Secrets: get_Secrets::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricSecretsResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceDescriptionResult_Impl: Sized {
    fn get_Description(&self) -> *mut super::super::FABRIC_SERVICE_DESCRIPTION;
}
impl ::windows_core::RuntimeName for IFabricServiceDescriptionResult {}
impl IFabricServiceDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceDescriptionResult_Vtbl {
        unsafe extern "system" fn get_Description<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Description()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceDescriptionResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceEndpointsVersion_Impl: Sized {
    fn Compare(
        &self,
        other: ::core::option::Option<&IFabricServiceEndpointsVersion>,
    ) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IFabricServiceEndpointsVersion {}
impl IFabricServiceEndpointsVersion_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceEndpointsVersion_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceEndpointsVersion_Vtbl {
        unsafe extern "system" fn Compare<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceEndpointsVersion_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            other: *mut ::core::ffi::c_void,
            compareresult: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Compare(::windows_core::from_raw_borrowed(&other)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compareresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compare: Compare::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceEndpointsVersion as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceGroupDescriptionResult_Impl: Sized {
    fn get_Description(&self) -> *mut super::super::FABRIC_SERVICE_GROUP_DESCRIPTION;
}
impl ::windows_core::RuntimeName for IFabricServiceGroupDescriptionResult {}
impl IFabricServiceGroupDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupDescriptionResult_Vtbl {
        unsafe extern "system" fn get_Description<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_GROUP_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Description()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceGroupDescriptionResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceGroupManagementClient_Impl: Sized {
    fn BeginCreateServiceGroup(
        &self,
        description: *const super::super::FABRIC_SERVICE_GROUP_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceGroup(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginDeleteServiceGroup(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteServiceGroup(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetServiceGroupDescription(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceGroupDescription(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricServiceGroupDescriptionResult>;
}
impl ::windows_core::RuntimeName for IFabricServiceGroupManagementClient {}
impl IFabricServiceGroupManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateServiceGroup<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_SERVICE_GROUP_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceGroup(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateServiceGroup<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceGroup(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteServiceGroup<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteServiceGroup(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteServiceGroup<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteServiceGroup(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetServiceGroupDescription<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceGroupDescription(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceGroupDescription<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceGroupDescription(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateServiceGroup: BeginCreateServiceGroup::<Identity, Impl, OFFSET>,
            EndCreateServiceGroup: EndCreateServiceGroup::<Identity, Impl, OFFSET>,
            BeginDeleteServiceGroup: BeginDeleteServiceGroup::<Identity, Impl, OFFSET>,
            EndDeleteServiceGroup: EndDeleteServiceGroup::<Identity, Impl, OFFSET>,
            BeginGetServiceGroupDescription: BeginGetServiceGroupDescription::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetServiceGroupDescription: EndGetServiceGroupDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceGroupManagementClient2_Impl:
    Sized + IFabricServiceGroupManagementClient_Impl
{
    fn BeginUpdateServiceGroup(
        &self,
        name: *const u16,
        servicegroupupdatedescription: *const super::super::FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateServiceGroup(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricServiceGroupManagementClient2 {}
impl IFabricServiceGroupManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient2_Vtbl {
        unsafe extern "system" fn BeginUpdateServiceGroup<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            servicegroupupdatedescription : *const super::super:: FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateServiceGroup(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&servicegroupupdatedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateServiceGroup<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateServiceGroup(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceGroupManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateServiceGroup: BeginUpdateServiceGroup::<Identity, Impl, OFFSET>,
            EndUpdateServiceGroup: EndUpdateServiceGroup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceGroupManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceGroupManagementClient3_Impl:
    Sized + IFabricServiceGroupManagementClient2_Impl
{
    fn BeginCreateServiceGroupFromTemplate(
        &self,
        applicationname: *const u16,
        servicename: *const u16,
        servicetypename: &::windows_core::PCWSTR,
        initializationdatasize: u32,
        initializationdata: *const u8,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceGroupFromTemplate(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricServiceGroupManagementClient3 {}
impl IFabricServiceGroupManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient3_Vtbl {
        unsafe extern "system" fn BeginCreateServiceGroupFromTemplate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            servicename: *const u16,
            servicetypename: ::windows_core::PCWSTR,
            initializationdatasize: u32,
            initializationdata: *const u8,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceGroupFromTemplate(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute_copy(&initializationdatasize),
                ::core::mem::transmute_copy(&initializationdata),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateServiceGroupFromTemplate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceGroupFromTemplate(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceGroupManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginCreateServiceGroupFromTemplate: BeginCreateServiceGroupFromTemplate::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndCreateServiceGroupFromTemplate: EndCreateServiceGroupFromTemplate::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceGroupManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceGroupManagementClient2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceGroupManagementClient4_Impl:
    Sized + IFabricServiceGroupManagementClient3_Impl
{
    fn BeginCreateServiceGroupFromTemplate2(
        &self,
        servicegroupfromtemplatedescription : *const super::super:: FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceGroupFromTemplate2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricServiceGroupManagementClient4 {}
impl IFabricServiceGroupManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient4_Vtbl {
        unsafe extern "system" fn BeginCreateServiceGroupFromTemplate2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicegroupfromtemplatedescription : *const super::super:: FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceGroupFromTemplate2(
                ::core::mem::transmute_copy(&servicegroupfromtemplatedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateServiceGroupFromTemplate2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceGroupFromTemplate2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceGroupManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginCreateServiceGroupFromTemplate2: BeginCreateServiceGroupFromTemplate2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndCreateServiceGroupFromTemplate2: EndCreateServiceGroupFromTemplate2::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceGroupManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceGroupManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceGroupManagementClient3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricServiceHealthResult_Impl: Sized {
    fn get_ServiceHealth(&self) -> *mut super::super::FABRIC_SERVICE_HEALTH;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricServiceHealthResult {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricServiceHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceHealthResult_Vtbl {
        unsafe extern "system" fn get_ServiceHealth<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceHealth()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceHealth: get_ServiceHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceHealthResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceManagementClient_Impl: Sized {
    fn BeginCreateService(
        &self,
        description: *const super::super::FABRIC_SERVICE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateService(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginCreateServiceFromTemplate(
        &self,
        applicationname: *const u16,
        servicename: *const u16,
        servicetypename: &::windows_core::PCWSTR,
        initializationdatasize: u32,
        initializationdata: *const u8,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceFromTemplate(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginDeleteService(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteService(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetServiceDescription(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceDescription(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricServiceDescriptionResult>;
    fn RegisterServicePartitionResolutionChangeHandler(
        &self,
        name: *const u16,
        keytype: super::super::FABRIC_PARTITION_KEY_TYPE,
        partitionkey: *const ::core::ffi::c_void,
        callback: ::core::option::Option<&IFabricServicePartitionResolutionChangeHandler>,
    ) -> ::windows_core::Result<i64>;
    fn UnregisterServicePartitionResolutionChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()>;
    fn BeginResolveServicePartition(
        &self,
        name: *const u16,
        partitionkeytype: super::super::FABRIC_PARTITION_KEY_TYPE,
        partitionkey: *const ::core::ffi::c_void,
        previousresult: ::core::option::Option<&IFabricResolvedServicePartitionResult>,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndResolveServicePartition(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricResolvedServicePartitionResult>;
}
impl ::windows_core::RuntimeName for IFabricServiceManagementClient {}
impl IFabricServiceManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateService<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_SERVICE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateService(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateService<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateService(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginCreateServiceFromTemplate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            servicename: *const u16,
            servicetypename: ::windows_core::PCWSTR,
            initializationdatasize: u32,
            initializationdata: *const u8,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceFromTemplate(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute_copy(&initializationdatasize),
                ::core::mem::transmute_copy(&initializationdata),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateServiceFromTemplate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceFromTemplate(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteService<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteService(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteService<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteService(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetServiceDescription<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceDescription(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceDescription<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceDescription(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterServicePartitionResolutionChangeHandler<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            keytype: super::super::FABRIC_PARTITION_KEY_TYPE,
            partitionkey: *const ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            callbackhandle: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterServicePartitionResolutionChangeHandler(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&keytype),
                ::core::mem::transmute_copy(&partitionkey),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callbackhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterServicePartitionResolutionChangeHandler<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callbackhandle: i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterServicePartitionResolutionChangeHandler(::core::mem::transmute_copy(
                &callbackhandle,
            ))
            .into()
        }
        unsafe extern "system" fn BeginResolveServicePartition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            partitionkeytype: super::super::FABRIC_PARTITION_KEY_TYPE,
            partitionkey: *const ::core::ffi::c_void,
            previousresult: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginResolveServicePartition(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&partitionkeytype),
                ::core::mem::transmute_copy(&partitionkey),
                ::windows_core::from_raw_borrowed(&previousresult),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndResolveServicePartition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndResolveServicePartition(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateService: BeginCreateService::<Identity, Impl, OFFSET>,
            EndCreateService: EndCreateService::<Identity, Impl, OFFSET>,
            BeginCreateServiceFromTemplate: BeginCreateServiceFromTemplate::<Identity, Impl, OFFSET>,
            EndCreateServiceFromTemplate: EndCreateServiceFromTemplate::<Identity, Impl, OFFSET>,
            BeginDeleteService: BeginDeleteService::<Identity, Impl, OFFSET>,
            EndDeleteService: EndDeleteService::<Identity, Impl, OFFSET>,
            BeginGetServiceDescription: BeginGetServiceDescription::<Identity, Impl, OFFSET>,
            EndGetServiceDescription: EndGetServiceDescription::<Identity, Impl, OFFSET>,
            RegisterServicePartitionResolutionChangeHandler:
                RegisterServicePartitionResolutionChangeHandler::<Identity, Impl, OFFSET>,
            UnregisterServicePartitionResolutionChangeHandler:
                UnregisterServicePartitionResolutionChangeHandler::<Identity, Impl, OFFSET>,
            BeginResolveServicePartition: BeginResolveServicePartition::<Identity, Impl, OFFSET>,
            EndResolveServicePartition: EndResolveServicePartition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceManagementClient2_Impl:
    Sized + IFabricServiceManagementClient_Impl
{
    fn BeginGetServiceManifest(
        &self,
        applicationtypename: &::windows_core::PCWSTR,
        applicationtypeversion: &::windows_core::PCWSTR,
        servicemanifestname: &::windows_core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceManifest(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<super::IFabricStringResult>;
    fn BeginUpdateService(
        &self,
        name: *const u16,
        serviceupdatedescription: *const super::super::FABRIC_SERVICE_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateService(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricServiceManagementClient2 {}
impl IFabricServiceManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient2_Vtbl {
        unsafe extern "system" fn BeginGetServiceManifest<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows_core::PCWSTR,
            applicationtypeversion: ::windows_core::PCWSTR,
            servicemanifestname: ::windows_core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceManifest(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
                ::core::mem::transmute(&servicemanifestname),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceManifest<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceManifest(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateService<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            serviceupdatedescription: *const super::super::FABRIC_SERVICE_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateService(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&serviceupdatedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateService<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateService(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetServiceManifest: BeginGetServiceManifest::<Identity, Impl, OFFSET>,
            EndGetServiceManifest: EndGetServiceManifest::<Identity, Impl, OFFSET>,
            BeginUpdateService: BeginUpdateService::<Identity, Impl, OFFSET>,
            EndUpdateService: EndUpdateService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceManagementClient3_Impl:
    Sized + IFabricServiceManagementClient2_Impl
{
    fn BeginRemoveReplica(
        &self,
        description: *const super::super::FABRIC_REMOVE_REPLICA_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRemoveReplica(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginRestartReplica(
        &self,
        description: *const super::super::FABRIC_RESTART_REPLICA_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartReplica(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricServiceManagementClient3 {}
impl IFabricServiceManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient3_Vtbl {
        unsafe extern "system" fn BeginRemoveReplica<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_REMOVE_REPLICA_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRemoveReplica(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRemoveReplica<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRemoveReplica(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginRestartReplica<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_RESTART_REPLICA_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartReplica(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRestartReplica<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRestartReplica(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRemoveReplica: BeginRemoveReplica::<Identity, Impl, OFFSET>,
            EndRemoveReplica: EndRemoveReplica::<Identity, Impl, OFFSET>,
            BeginRestartReplica: BeginRestartReplica::<Identity, Impl, OFFSET>,
            EndRestartReplica: EndRestartReplica::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceManagementClient4_Impl:
    Sized + IFabricServiceManagementClient3_Impl
{
    fn BeginRegisterServiceNotificationFilter(
        &self,
        description: *const super::super::FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndRegisterServiceNotificationFilter(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<i64>;
    fn BeginUnregisterServiceNotificationFilter(
        &self,
        filterid: i64,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnregisterServiceNotificationFilter(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricServiceManagementClient4 {}
impl IFabricServiceManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient4_Vtbl {
        unsafe extern "system" fn BeginRegisterServiceNotificationFilter<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRegisterServiceNotificationFilter(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRegisterServiceNotificationFilter<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            filterid: *mut i64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndRegisterServiceNotificationFilter(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filterid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUnregisterServiceNotificationFilter<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            filterid: i64,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnregisterServiceNotificationFilter(
                ::core::mem::transmute_copy(&filterid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnregisterServiceNotificationFilter<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnregisterServiceNotificationFilter(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRegisterServiceNotificationFilter: BeginRegisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRegisterServiceNotificationFilter: EndRegisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginUnregisterServiceNotificationFilter: BeginUnregisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUnregisterServiceNotificationFilter: EndUnregisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricServiceManagementClient5_Impl:
    Sized + IFabricServiceManagementClient4_Impl
{
    fn BeginDeleteService2(
        &self,
        deletedescription: *const super::super::FABRIC_DELETE_SERVICE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteService2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricServiceManagementClient5 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricServiceManagementClient5_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient5_Vtbl {
        unsafe extern "system" fn BeginDeleteService2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            deletedescription: *const super::super::FABRIC_DELETE_SERVICE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteService2(
                ::core::mem::transmute_copy(&deletedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDeleteService2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteService2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeleteService2: BeginDeleteService2::<Identity, Impl, OFFSET>,
            EndDeleteService2: EndDeleteService2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient5 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient4 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricServiceManagementClient6_Impl:
    Sized + IFabricServiceManagementClient5_Impl
{
    fn BeginCreateServiceFromTemplate2(
        &self,
        servicefromtemplatedescription : *const super::super:: FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceFromTemplate2(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricServiceManagementClient6 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricServiceManagementClient6_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient6_Vtbl {
        unsafe extern "system" fn BeginCreateServiceFromTemplate2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicefromtemplatedescription : *const super::super:: FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceFromTemplate2(
                ::core::mem::transmute_copy(&servicefromtemplatedescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCreateServiceFromTemplate2<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceFromTemplate2(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginCreateServiceFromTemplate2: BeginCreateServiceFromTemplate2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndCreateServiceFromTemplate2: EndCreateServiceFromTemplate2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient6 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricServiceManagementClient5 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceNotification_Impl: Sized {
    fn get_Notification(&self) -> *mut super::super::FABRIC_SERVICE_NOTIFICATION;
    fn GetVersion(&self) -> ::windows_core::Result<IFabricServiceEndpointsVersion>;
}
impl ::windows_core::RuntimeName for IFabricServiceNotification {}
impl IFabricServiceNotification_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceNotification_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceNotification_Vtbl {
        unsafe extern "system" fn get_Notification<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceNotification_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_NOTIFICATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Notification()
        }
        unsafe extern "system" fn GetVersion<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceNotification_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Notification: get_Notification::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceNotification as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServiceNotificationEventHandler_Impl: Sized {
    fn OnNotification(
        &self,
        __midl__ifabricservicenotificationeventhandler0000: ::core::option::Option<
            &IFabricServiceNotification,
        >,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricServiceNotificationEventHandler {}
impl IFabricServiceNotificationEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceNotificationEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceNotificationEventHandler_Vtbl {
        unsafe extern "system" fn OnNotification<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceNotificationEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            __midl__ifabricservicenotificationeventhandler0000: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNotification(::windows_core::from_raw_borrowed(
                &__midl__ifabricservicenotificationeventhandler0000,
            ))
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnNotification: OnNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricServiceNotificationEventHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricServicePartitionResolutionChangeHandler_Impl: Sized {
    fn OnChange(
        &self,
        source: ::core::option::Option<&IFabricServiceManagementClient>,
        handlerid: i64,
        partition: ::core::option::Option<&IFabricResolvedServicePartitionResult>,
        error: ::windows_core::HRESULT,
    );
}
impl ::windows_core::RuntimeName for IFabricServicePartitionResolutionChangeHandler {}
impl IFabricServicePartitionResolutionChangeHandler_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServicePartitionResolutionChangeHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricServicePartitionResolutionChangeHandler_Vtbl {
        unsafe extern "system" fn OnChange<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServicePartitionResolutionChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            handlerid: i64,
            partition: *mut ::core::ffi::c_void,
            error: ::windows_core::HRESULT,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChange(
                ::windows_core::from_raw_borrowed(&source),
                ::core::mem::transmute_copy(&handlerid),
                ::windows_core::from_raw_borrowed(&partition),
                ::core::mem::transmute_copy(&error),
            )
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnChange: OnChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == & < IFabricServicePartitionResolutionChangeHandler < > as::windows_core::ComInterface >::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricStartNodeResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_NODE_RESULT;
}
impl ::windows_core::RuntimeName for IFabricStartNodeResult {}
impl IFabricStartNodeResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStartNodeResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStartNodeResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStartNodeResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricStartNodeResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricStopNodeResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_NODE_RESULT;
}
impl ::windows_core::RuntimeName for IFabricStopNodeResult {}
impl IFabricStopNodeResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStopNodeResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStopNodeResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStopNodeResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricStopNodeResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricTestCommandStatusResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::TEST_COMMAND_QUERY_RESULT_LIST;
}
impl ::windows_core::RuntimeName for IFabricTestCommandStatusResult {}
impl IFabricTestCommandStatusResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestCommandStatusResult_Impl,
        const OFFSET: isize,
    >() -> IFabricTestCommandStatusResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestCommandStatusResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::TEST_COMMAND_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricTestCommandStatusResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricTestManagementClient_Impl: Sized {
    fn BeginStartPartitionDataLoss(
        &self,
        invokedatalossdescription : *const super::super:: FABRIC_START_PARTITION_DATA_LOSS_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartPartitionDataLoss(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetPartitionDataLossProgress(
        &self,
        operationid: &::windows_core::GUID,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionDataLossProgress(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPartitionDataLossProgressResult>;
    fn BeginStartPartitionQuorumLoss(
        &self,
        invokequorumlossdescription : *const super::super:: FABRIC_START_PARTITION_QUORUM_LOSS_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartPartitionQuorumLoss(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetPartitionQuorumLossProgress(
        &self,
        operationid: &::windows_core::GUID,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionQuorumLossProgress(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPartitionQuorumLossProgressResult>;
    fn BeginStartPartitionRestart(
        &self,
        restartpartitiondescription : *const super::super:: FABRIC_START_PARTITION_RESTART_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartPartitionRestart(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetPartitionRestartProgress(
        &self,
        operationid: &::windows_core::GUID,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionRestartProgress(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricPartitionRestartProgressResult>;
    fn BeginGetTestCommandStatusList(
        &self,
        operationid: *const super::super::FABRIC_TEST_COMMAND_LIST_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetTestCommandStatusList(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricTestCommandStatusResult>;
    fn BeginCancelTestCommand(
        &self,
        invokedatalossdescription: *const super::super::FABRIC_CANCEL_TEST_COMMAND_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndCancelTestCommand(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricTestManagementClient {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricTestManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient_Vtbl {
        unsafe extern "system" fn BeginStartPartitionDataLoss<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            invokedatalossdescription : *const super::super:: FABRIC_START_PARTITION_DATA_LOSS_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartPartitionDataLoss(
                ::core::mem::transmute_copy(&invokedatalossdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartPartitionDataLoss<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartPartitionDataLoss(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPartitionDataLossProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionDataLossProgress(
                ::core::mem::transmute(&operationid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionDataLossProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionDataLossProgress(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStartPartitionQuorumLoss<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            invokequorumlossdescription : *const super::super:: FABRIC_START_PARTITION_QUORUM_LOSS_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartPartitionQuorumLoss(
                ::core::mem::transmute_copy(&invokequorumlossdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartPartitionQuorumLoss<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartPartitionQuorumLoss(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPartitionQuorumLossProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionQuorumLossProgress(
                ::core::mem::transmute(&operationid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionQuorumLossProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .EndGetPartitionQuorumLossProgress(::windows_core::from_raw_borrowed(&context))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStartPartitionRestart<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartpartitiondescription : *const super::super:: FABRIC_START_PARTITION_RESTART_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartPartitionRestart(
                ::core::mem::transmute_copy(&restartpartitiondescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartPartitionRestart<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartPartitionRestart(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPartitionRestartProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionRestartProgress(
                ::core::mem::transmute(&operationid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionRestartProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionRestartProgress(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetTestCommandStatusList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: *const super::super::FABRIC_TEST_COMMAND_LIST_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetTestCommandStatusList(
                ::core::mem::transmute_copy(&operationid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetTestCommandStatusList<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetTestCommandStatusList(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCancelTestCommand<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            invokedatalossdescription: *const super::super::FABRIC_CANCEL_TEST_COMMAND_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCancelTestCommand(
                ::core::mem::transmute_copy(&invokedatalossdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCancelTestCommand<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCancelTestCommand(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginStartPartitionDataLoss: BeginStartPartitionDataLoss::<Identity, Impl, OFFSET>,
            EndStartPartitionDataLoss: EndStartPartitionDataLoss::<Identity, Impl, OFFSET>,
            BeginGetPartitionDataLossProgress: BeginGetPartitionDataLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionDataLossProgress: EndGetPartitionDataLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginStartPartitionQuorumLoss: BeginStartPartitionQuorumLoss::<Identity, Impl, OFFSET>,
            EndStartPartitionQuorumLoss: EndStartPartitionQuorumLoss::<Identity, Impl, OFFSET>,
            BeginGetPartitionQuorumLossProgress: BeginGetPartitionQuorumLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionQuorumLossProgress: EndGetPartitionQuorumLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginStartPartitionRestart: BeginStartPartitionRestart::<Identity, Impl, OFFSET>,
            EndStartPartitionRestart: EndStartPartitionRestart::<Identity, Impl, OFFSET>,
            BeginGetPartitionRestartProgress: BeginGetPartitionRestartProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionRestartProgress: EndGetPartitionRestartProgress::<Identity, Impl, OFFSET>,
            BeginGetTestCommandStatusList: BeginGetTestCommandStatusList::<Identity, Impl, OFFSET>,
            EndGetTestCommandStatusList: EndGetTestCommandStatusList::<Identity, Impl, OFFSET>,
            BeginCancelTestCommand: BeginCancelTestCommand::<Identity, Impl, OFFSET>,
            EndCancelTestCommand: EndCancelTestCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricTestManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricTestManagementClient2_Impl: Sized + IFabricTestManagementClient_Impl {
    fn BeginStartChaos(
        &self,
        restartpartitiondescription: *const super::super::FABRIC_START_CHAOS_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartChaos(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginStopChaos(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStopChaos(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetChaosReport(
        &self,
        getchaosreportdescription: *const super::super::FABRIC_GET_CHAOS_REPORT_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaosReport(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricChaosReportResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricTestManagementClient2 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricTestManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient2_Vtbl {
        unsafe extern "system" fn BeginStartChaos<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartpartitiondescription: *const super::super::FABRIC_START_CHAOS_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartChaos(
                ::core::mem::transmute_copy(&restartpartitiondescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartChaos<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartChaos(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginStopChaos<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStopChaos(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStopChaos<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStopChaos(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetChaosReport<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            getchaosreportdescription: *const super::super::FABRIC_GET_CHAOS_REPORT_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaosReport(
                ::core::mem::transmute_copy(&getchaosreportdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetChaosReport<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaosReport(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricTestManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginStartChaos: BeginStartChaos::<Identity, Impl, OFFSET>,
            EndStartChaos: EndStartChaos::<Identity, Impl, OFFSET>,
            BeginStopChaos: BeginStopChaos::<Identity, Impl, OFFSET>,
            EndStopChaos: EndStopChaos::<Identity, Impl, OFFSET>,
            BeginGetChaosReport: BeginGetChaosReport::<Identity, Impl, OFFSET>,
            EndGetChaosReport: EndGetChaosReport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricTestManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricTestManagementClient as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricTestManagementClient3_Impl: Sized + IFabricTestManagementClient2_Impl {
    fn BeginStartNodeTransition(
        &self,
        description: *const super::super::FABRIC_NODE_TRANSITION_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartNodeTransition(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetNodeTransitionProgress(
        &self,
        operationid: &::windows_core::GUID,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeTransitionProgress(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricNodeTransitionProgressResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricTestManagementClient3 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricTestManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient3_Vtbl {
        unsafe extern "system" fn BeginStartNodeTransition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_NODE_TRANSITION_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartNodeTransition(
                ::core::mem::transmute_copy(&description),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStartNodeTransition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartNodeTransition(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetNodeTransitionProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeTransitionProgress(
                ::core::mem::transmute(&operationid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNodeTransitionProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeTransitionProgress(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricTestManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginStartNodeTransition: BeginStartNodeTransition::<Identity, Impl, OFFSET>,
            EndStartNodeTransition: EndStartNodeTransition::<Identity, Impl, OFFSET>,
            BeginGetNodeTransitionProgress: BeginGetNodeTransitionProgress::<Identity, Impl, OFFSET>,
            EndGetNodeTransitionProgress: EndGetNodeTransitionProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricTestManagementClient3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricTestManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricTestManagementClient2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricTestManagementClient4_Impl: Sized + IFabricTestManagementClient3_Impl {
    fn BeginGetChaos(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaos(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricChaosDescriptionResult>;
    fn BeginGetChaosSchedule(
        &self,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaosSchedule(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricChaosScheduleDescriptionResult>;
    fn BeginSetChaosSchedule(
        &self,
        setchaosscheduledescription: *const super::super::FABRIC_CHAOS_SERVICE_SCHEDULE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndSetChaosSchedule(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()>;
    fn BeginGetChaosEvents(
        &self,
        chaoseventsdescription: *const super::super::FABRIC_CHAOS_EVENTS_SEGMENT_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaosEvents(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricChaosEventsSegmentResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricTestManagementClient4 {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricTestManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient4_Vtbl {
        unsafe extern "system" fn BeginGetChaos<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaos(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetChaos<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaos(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetChaosSchedule<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaosSchedule(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetChaosSchedule<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaosSchedule(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSetChaosSchedule<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            setchaosscheduledescription : *const super::super:: FABRIC_CHAOS_SERVICE_SCHEDULE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSetChaosSchedule(
                ::core::mem::transmute_copy(&setchaosscheduledescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSetChaosSchedule<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSetChaosSchedule(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetChaosEvents<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            chaoseventsdescription: *const super::super::FABRIC_CHAOS_EVENTS_SEGMENT_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaosEvents(
                ::core::mem::transmute_copy(&chaoseventsdescription),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::windows_core::from_raw_borrowed(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetChaosEvents<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaosEvents(::windows_core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricTestManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetChaos: BeginGetChaos::<Identity, Impl, OFFSET>,
            EndGetChaos: EndGetChaos::<Identity, Impl, OFFSET>,
            BeginGetChaosSchedule: BeginGetChaosSchedule::<Identity, Impl, OFFSET>,
            EndGetChaosSchedule: EndGetChaosSchedule::<Identity, Impl, OFFSET>,
            BeginSetChaosSchedule: BeginSetChaosSchedule::<Identity, Impl, OFFSET>,
            EndSetChaosSchedule: EndSetChaosSchedule::<Identity, Impl, OFFSET>,
            BeginGetChaosEvents: BeginGetChaosEvents::<Identity, Impl, OFFSET>,
            EndGetChaosEvents: EndGetChaosEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricTestManagementClient4 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricTestManagementClient as ::windows_core::ComInterface>::IID
            || iid == &<IFabricTestManagementClient2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricTestManagementClient3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricUpgradeOrchestrationServiceStateResult_Impl: Sized {
    fn get_State(&self) -> *mut super::super::FABRIC_UPGRADE_ORCHESTRATION_SERVICE_STATE;
}
impl ::windows_core::RuntimeName for IFabricUpgradeOrchestrationServiceStateResult {}
impl IFabricUpgradeOrchestrationServiceStateResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeOrchestrationServiceStateResult_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeOrchestrationServiceStateResult_Vtbl {
        unsafe extern "system" fn get_State<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeOrchestrationServiceStateResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_UPGRADE_ORCHESTRATION_SERVICE_STATE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_State()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_State: get_State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricUpgradeOrchestrationServiceStateResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricUpgradeProgressResult_Impl: Sized {
    fn get_TargetCodeVersion(&self) -> ::windows_core::PCWSTR;
    fn get_TargetConfigVersion(&self) -> ::windows_core::PCWSTR;
    fn get_UpgradeState(&self) -> super::super::FABRIC_UPGRADE_STATE;
    fn GetUpgradeDomains(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows_core::Result<()>;
    fn GetChangedUpgradeDomains(
        &self,
        previousprogress: ::core::option::Option<&IFabricUpgradeProgressResult>,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricUpgradeProgressResult {}
impl IFabricUpgradeProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeProgressResult_Vtbl {
        unsafe extern "system" fn get_TargetCodeVersion<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TargetCodeVersion()
        }
        unsafe extern "system" fn get_TargetConfigVersion<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TargetConfigVersion()
        }
        unsafe extern "system" fn get_UpgradeState<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_UPGRADE_STATE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeState()
        }
        unsafe extern "system" fn GetUpgradeDomains<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpgradeDomains(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        unsafe extern "system" fn GetChangedUpgradeDomains<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            previousprogress: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChangedUpgradeDomains(
                ::windows_core::from_raw_borrowed(&previousprogress),
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_TargetCodeVersion: get_TargetCodeVersion::<Identity, Impl, OFFSET>,
            get_TargetConfigVersion: get_TargetConfigVersion::<Identity, Impl, OFFSET>,
            get_UpgradeState: get_UpgradeState::<Identity, Impl, OFFSET>,
            GetUpgradeDomains: GetUpgradeDomains::<Identity, Impl, OFFSET>,
            GetChangedUpgradeDomains: GetChangedUpgradeDomains::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricUpgradeProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricUpgradeProgressResult2_Impl: Sized + IFabricUpgradeProgressResult_Impl {
    fn get_RollingUpgradeMode(&self) -> super::super::FABRIC_ROLLING_UPGRADE_MODE;
    fn get_NextUpgradeDomain(&self) -> ::windows_core::PCWSTR;
}
impl ::windows_core::RuntimeName for IFabricUpgradeProgressResult2 {}
impl IFabricUpgradeProgressResult2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeProgressResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeProgressResult2_Vtbl {
        unsafe extern "system" fn get_RollingUpgradeMode<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ROLLING_UPGRADE_MODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_RollingUpgradeMode()
        }
        unsafe extern "system" fn get_NextUpgradeDomain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NextUpgradeDomain()
        }
        Self {
            base__: IFabricUpgradeProgressResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_RollingUpgradeMode: get_RollingUpgradeMode::<Identity, Impl, OFFSET>,
            get_NextUpgradeDomain: get_NextUpgradeDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricUpgradeProgressResult2 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricUpgradeProgressResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon_FabricClient\"`, `\"implement\"`*"]
pub trait IFabricUpgradeProgressResult3_Impl: Sized + IFabricUpgradeProgressResult2_Impl {
    fn get_UpgradeProgress(&self) -> *mut super::super::FABRIC_UPGRADE_PROGRESS;
}
impl ::windows_core::RuntimeName for IFabricUpgradeProgressResult3 {}
impl IFabricUpgradeProgressResult3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeProgressResult3_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeProgressResult3_Vtbl {
        unsafe extern "system" fn get_UpgradeProgress<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_UPGRADE_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeProgress()
        }
        Self {
            base__: IFabricUpgradeProgressResult2_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_UpgradeProgress: get_UpgradeProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricUpgradeProgressResult3 as ::windows_core::ComInterface>::IID
            || iid == &<IFabricUpgradeProgressResult as ::windows_core::ComInterface>::IID
            || iid == &<IFabricUpgradeProgressResult2 as ::windows_core::ComInterface>::IID
    }
}
