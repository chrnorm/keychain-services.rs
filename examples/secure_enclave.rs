use keychain_services as ks;

fn main() {
    println!("Hello, world!");
    let keypair = generate_keypair("VXC4RPMSUJ.com.chrisnorman.sekey1", "supplylock-dev");
    println!("{:?}", keypair);
    let data = b"hello world";
    let sig = keypair
        .private_key
        .sign(ks::KeyAlgorithm::ECDSASignatureMessageX962SHA256, data)
        .unwrap();

    println!("{:?}", sig);

    let public_key_bytes = keypair.public_key.to_external_representation().unwrap();

    let res = ring::signature::verify(
        &ring::signature::ECDSA_P256_SHA256_ASN1,
        untrusted::Input::from(&public_key_bytes),
        untrusted::Input::from(data),
        untrusted::Input::from(sig.as_ref()),
    )
    .unwrap();
    println!("{:?}", res);

    let key = key_query("VXC4RPMSUJ.com.chrisnorman.sekey1");
    println!("{:?}", key);
    // let bytes = data.as_bytes();
    let result = key
        .sign(ks::KeyAlgorithm::ECDSASignatureMessageX962SHA256, data)
        .unwrap();

    let res = ring::signature::verify(
        &ring::signature::ECDSA_P256_SHA256_ASN1,
        untrusted::Input::from(&public_key_bytes),
        untrusted::Input::from(data),
        untrusted::Input::from(result.as_ref()),
    )
    .unwrap();
    println!("{:?}", res);

    // match result {
    //     Err(_) => {
    //         panic!("Could not sign");
    //     }
    //     Ok(sig) => {
    //         let verify_result = keypair.public_key.verify(data.as_bytes(), &sig);
    //         println!("{:?}", verify_result);
    //     }
    // };
}

fn generate_keypair(tag: &str, label: &str) -> ks::KeyPair {
    let mut flags = ks::AccessControlFlags::new();
    flags.add(ks::AccessOption::PrivateKeyUsage);
    // flags.add(ks::AccessConstraint::BiometryAny);

    let acl =
        ks::AccessControl::create_with_flags(ks::AttrAccessible::WhenUnlockedThisDeviceOnly, flags)
            .unwrap();

    let generate_params = ks::KeyPairGenerateParams::new(ks::AttrKeyType::EcSecPrimeRandom, 256)
        .access_control(&acl)
        .token_id(ks::AttrTokenId::SecureEnclave)
        .application_tag(tag)
        .label(label)
        .permanent(true);

    ks::KeyPair::generate(generate_params).unwrap()
}

fn key_query(label: &str) -> ks::Key {
    let private_key_query = ks::keychain::item::Query::new()
        .key_class(ks::AttrKeyClass::Private)
        .token_id(ks::AttrTokenId::SecureEnclave)
        .key_type(ks::AttrKeyType::EcSecPrimeRandom)
        .application_tag(label);

    let private_key = ks::Key::find(private_key_query).unwrap();

    private_key
}
