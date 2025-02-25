#[inline]
pub unsafe fn McastApiCleanup() {
    windows_targets::link!("dhcpcsvc.dll" "system" fn McastApiCleanup());
    McastApiCleanup()
}
#[inline]
pub unsafe fn McastApiStartup(version: *mut u32) -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn McastApiStartup(version : *mut u32) -> u32);
    McastApiStartup(version)
}
#[inline]
pub unsafe fn McastEnumerateScopes<P0>(addrfamily: u16, requery: P0, pscopelist: *mut MCAST_SCOPE_ENTRY, pscopelen: *mut u32, pscopecount: *mut u32) -> u32
where
    P0: windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpcsvc.dll" "system" fn McastEnumerateScopes(addrfamily : u16, requery : super::super::Foundation:: BOOL, pscopelist : *mut MCAST_SCOPE_ENTRY, pscopelen : *mut u32, pscopecount : *mut u32) -> u32);
    McastEnumerateScopes(addrfamily, requery.into_param().abi(), pscopelist, pscopelen, pscopecount)
}
#[inline]
pub unsafe fn McastGenUID(prequestid: *mut MCAST_CLIENT_UID) -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn McastGenUID(prequestid : *mut MCAST_CLIENT_UID) -> u32);
    McastGenUID(prequestid)
}
#[inline]
pub unsafe fn McastReleaseAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, preleaserequest: *mut MCAST_LEASE_REQUEST) -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn McastReleaseAddress(addrfamily : u16, prequestid : *mut MCAST_CLIENT_UID, preleaserequest : *mut MCAST_LEASE_REQUEST) -> u32);
    McastReleaseAddress(addrfamily, prequestid, preleaserequest)
}
#[inline]
pub unsafe fn McastRenewAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, prenewrequest: *mut MCAST_LEASE_REQUEST, prenewresponse: *mut MCAST_LEASE_RESPONSE) -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn McastRenewAddress(addrfamily : u16, prequestid : *mut MCAST_CLIENT_UID, prenewrequest : *mut MCAST_LEASE_REQUEST, prenewresponse : *mut MCAST_LEASE_RESPONSE) -> u32);
    McastRenewAddress(addrfamily, prequestid, prenewrequest, prenewresponse)
}
#[inline]
pub unsafe fn McastRequestAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, pscopectx: *mut MCAST_SCOPE_CTX, paddrrequest: *mut MCAST_LEASE_REQUEST, paddrresponse: *mut MCAST_LEASE_RESPONSE) -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn McastRequestAddress(addrfamily : u16, prequestid : *mut MCAST_CLIENT_UID, pscopectx : *mut MCAST_SCOPE_CTX, paddrrequest : *mut MCAST_LEASE_REQUEST, paddrresponse : *mut MCAST_LEASE_RESPONSE) -> u32);
    McastRequestAddress(addrfamily, prequestid, pscopectx, paddrrequest, paddrresponse)
}
pub const MCAST_API_CURRENT_VERSION: i32 = 1i32;
pub const MCAST_API_VERSION_0: i32 = 0i32;
pub const MCAST_API_VERSION_1: i32 = 1i32;
pub const MCAST_CLIENT_ID_LEN: u32 = 17u32;
#[repr(C)]
pub union IPNG_ADDRESS {
    pub IpAddrV4: u32,
    pub IpAddrV6: [u8; 16],
}
impl Copy for IPNG_ADDRESS {}
impl Clone for IPNG_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for IPNG_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for IPNG_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MCAST_CLIENT_UID {
    pub ClientUID: *mut u8,
    pub ClientUIDLength: u32,
}
impl Copy for MCAST_CLIENT_UID {}
impl Clone for MCAST_CLIENT_UID {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MCAST_CLIENT_UID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MCAST_CLIENT_UID").field("ClientUID", &self.ClientUID).field("ClientUIDLength", &self.ClientUIDLength).finish()
    }
}
impl windows_core::TypeKind for MCAST_CLIENT_UID {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MCAST_CLIENT_UID {
    fn eq(&self, other: &Self) -> bool {
        self.ClientUID == other.ClientUID && self.ClientUIDLength == other.ClientUIDLength
    }
}
impl Eq for MCAST_CLIENT_UID {}
impl Default for MCAST_CLIENT_UID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MCAST_LEASE_REQUEST {
    pub LeaseStartTime: i32,
    pub MaxLeaseStartTime: i32,
    pub LeaseDuration: u32,
    pub MinLeaseDuration: u32,
    pub ServerAddress: IPNG_ADDRESS,
    pub MinAddrCount: u16,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl Copy for MCAST_LEASE_REQUEST {}
impl Clone for MCAST_LEASE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for MCAST_LEASE_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for MCAST_LEASE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MCAST_LEASE_RESPONSE {
    pub LeaseStartTime: i32,
    pub LeaseEndTime: i32,
    pub ServerAddress: IPNG_ADDRESS,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl Copy for MCAST_LEASE_RESPONSE {}
impl Clone for MCAST_LEASE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for MCAST_LEASE_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for MCAST_LEASE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MCAST_SCOPE_CTX {
    pub ScopeID: IPNG_ADDRESS,
    pub Interface: IPNG_ADDRESS,
    pub ServerID: IPNG_ADDRESS,
}
impl Copy for MCAST_SCOPE_CTX {}
impl Clone for MCAST_SCOPE_CTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for MCAST_SCOPE_CTX {
    type TypeKind = windows_core::CopyType;
}
impl Default for MCAST_SCOPE_CTX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MCAST_SCOPE_ENTRY {
    pub ScopeCtx: MCAST_SCOPE_CTX,
    pub LastAddr: IPNG_ADDRESS,
    pub TTL: u32,
    pub ScopeDesc: super::super::Foundation::UNICODE_STRING,
}
impl Copy for MCAST_SCOPE_ENTRY {}
impl Clone for MCAST_SCOPE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for MCAST_SCOPE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MCAST_SCOPE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
