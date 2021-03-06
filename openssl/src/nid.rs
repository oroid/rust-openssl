#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(usize)]
pub enum Nid {
    Undefined,
    Rsadsi,
    Pkcs,
    MD2,
    MD4,
    MD5,
    RC4,
    RsaEncryption,
    RSA_MD2,
    RSA_MD5,
    PBE_MD2_DES,
    X500,
    x509,
    CN,
    C,
    L,
    ST,
    O,
    OU,
    RSA,
    Pkcs7,
    Pkcs7_data,
    Pkcs7_signedData,
    Pkcs7_envelopedData,
    Pkcs7_signedAndEnvelopedData,
    Pkcs7_digestData,
    Pkcs7_encryptedData,
    Pkcs3,
    DhKeyAgreement,
    DES_ECB,
    DES_CFB,
    DES_CBC,
    DES_EDE,
    DES_EDE3,
    IDEA_CBC,
    IDEA_ECB,
    RC2_CBC,
    RC2_ECB,
    RC2_CFB,
    RC2_OFB,
    SHA,
    RSA_SHA,
    DES_EDE_CBC,
    DES_EDE3_CBC,
    DES_OFB,
    IDEA_OFB,
    Pkcs9,
    Email,
    UnstructuredName,
    ContentType,
    MessageDigest,
    SigningTime,
    CounterSignature,
    UnstructuredAddress,
    ExtendedCertificateAttributes,
    Netscape,
    NetscapeCertExtention,
    NetscapeDatatype,
    DES_EDE_CFB64,
    DES_EDE3_CFB64,
    DES_EDE_OFB64,
    DES_EDE3_OFB64,
    SHA1,
    RSA_SHA1,
    DSA_SHA,
    DSA_OLD,
    PBE_SHA1_RC2_64,
    PBKDF2,
    DSA_SHA1_OLD,
    NetscapeCertType,
    NetscapeBaseUrl,
    NetscapeRevocationUrl,
    NetscapeCARevocationUrl,
    NetscapeRenewalUrl,
    NetscapeCAPolicyUrl,
    NetscapeSSLServerName,
    NetscapeComment,
    NetscapeCertSequence,
    DESX_CBC,
    ID_CE,
    SubjectKeyIdentifier,
    KeyUsage,
    PrivateKeyUsagePeriod,
    SubjectAltName,
    IssuerAltName,
    BasicConstraints,
    CrlNumber,
    CertificatePolicies,
    AuthorityKeyIdentifier,
    BF_CBC,
    BF_ECB,
    BF_OFB,
    MDC2,
    RSA_MDC2,
    RC4_40,
    RC2_40_CBC,
    G,
    S,
    I,
    UID,
    CrlDistributionPoints,
    RSA_NP_MD5,
    SN,
    T,
    D,
    CAST5_CBC,
    CAST5_ECB,
    CAST5_CFB,
    CAST5_OFB,
    PbeWithMD5AndCast5CBC,
    DSA_SHA1,
    MD5_SHA1,
    RSA_SHA1_2,
    DSA,
    RIPEMD160,
    RSA_RIPEMD160,
    RC5_CBC,
    RC5_ECB,
    RC5_CFB,
    RC5_OFB,
    RLE,
    ZLIB,
    ExtendedKeyUsage,
    PKIX,
    ID_KP,
    ServerAuth,
    ClientAuth,
    CodeSigning,
    EmailProtection,
    TimeStamping,
    MsCodeInd,
    MsCodeCom,
    MsCtlSigh,
    MsSGC,
    MsEFS,
    NsSGC,
    DeltaCRL,
    CRLReason,
    InvalidityDate,
    SXNetID,
    Pkcs12,
    PBE_SHA1_RC4_128,
    PBE_SHA1_RC4_40,
    PBE_SHA1_3DES,
    PBE_SHA1_2DES,
    PBE_SHA1_RC2_128,
    PBE_SHA1_RC2_40,
    KeyBag,
    Pkcs8ShroudedKeyBag,
    CertBag,
    CrlBag,
    SecretBag,
    SafeContentsBag,
    FriendlyName,
    LocalKeyID,
    X509Certificate,
    SdsiCertificate,
    X509Crl,
    PBES2,
    PBMAC1,
    HmacWithSha1,
    ID_QT_CPS,
    ID_QT_UNOTICE,
    RC2_64_CBC,
    SMIMECaps
}
